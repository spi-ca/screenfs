//! ScreenFS binary entrypoint.
//!
//! The binary validates launch inputs, builds runtime config, sets stable FUSE
//! mount options, and runs the `fractal-fuse` session with no fallback mount path.

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use fractal_fuse::{MountOptions, Session, SessionShutdownHandle};
use screenfs::{LaunchArgs, RuntimeConfig, ScreenFs, ensure_non_root_user};

// Keep startup validation before mount/session creation so failures are explicit
// and no partial mount state is left behind.
fn main() -> std::io::Result<()> {
    let argv = std::env::args_os().collect::<Vec<_>>();
    if LaunchArgs::wants_help(argv.iter().cloned()) {
        println!("{}", LaunchArgs::help());
        std::process::exit(0);
    }

    if let Err(error) = ensure_non_root_user() {
        eprintln!("{error}");
        std::process::exit(2);
    }

    let args = match LaunchArgs::parse_from(argv) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    if !args.cli.source_root.is_dir() {
        eprintln!(
            "source root is not a directory: {}",
            args.cli.source_root.display()
        );
        std::process::exit(2);
    }
    if !args.cli.mount_root.is_dir() {
        eprintln!(
            "mount root is not a directory: {}",
            args.cli.mount_root.display()
        );
        std::process::exit(2);
    }

    let experimental_writeback_cache = args.experimental_writeback_cache;
    let cfg = RuntimeConfig::from_launch(args)
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;

    eprintln!(
        "mounting screenfs: source={} mount={} visibility-default={} visibility-source={} hidden-rules={} visible-rules={} mutability-default={} mutability-source={} readonly-rules={} writable-rules={} io_uring=required writeback-cache={}",
        cfg.source_root.display(),
        cfg.mount_root.display(),
        cfg.visibility_default().as_str(),
        cfg.visibility_source().as_str(),
        cfg.hidden_rule_count(),
        cfg.visible_rule_count(),
        cfg.mutability_default().as_str(),
        cfg.mutability_source().as_str(),
        cfg.readonly_rule_count(),
        cfg.writable_rule_count(),
        if experimental_writeback_cache {
            "experimental"
        } else {
            "off"
        }
    );

    let opts = screenfs_mount_options(experimental_writeback_cache);
    if experimental_writeback_cache {
        eprintln!(
            "warning: --experimental-writeback-cache is opt-in smoke/benchmark surface only; kernel cache invalidation is not supported and O_WRONLY read isolation is not a security boundary"
        );
    }

    let mount_root = cfg.mount_root.clone();
    let mount_root_for_mountinfo = normalize_mount_root_for_mountinfo(&mount_root);
    let shutdown_signals = block_shutdown_signals()?;
    let shutdown_signal_state = ShutdownSignalState::new();
    let _shutdown_signal_thread =
        spawn_shutdown_signal_forwarder(shutdown_signals, shutdown_signal_state.clone())?;
    let session = Session::new(mount_root.clone(), opts)?;
    shutdown_signal_state.install_shutdown(session.shutdown_handle());

    // `fractal-fuse = 0.4.0` enforces `FUSE_OVER_IO_URING` during
    // `Session::run`'s `FUSE_INIT` negotiation. Preserve the current
    // fail-fast contract by surfacing that error directly with no fallback.
    // `Session::run` also performs the normal `fusermount3 -u` teardown when
    // the serve loop exits. ScreenFS reports stale mounts after return instead
    // of silently attempting lazy unmount.
    let result = session.run(ScreenFs::new(cfg));
    if let Err(error) = report_stale_mount_guidance(&mount_root_for_mountinfo) {
        eprintln!(
            "failed to inspect mount state after ScreenFS shutdown for {}: {error}",
            mount_root_for_mountinfo.display()
        );
    }
    result
}

fn block_shutdown_signals() -> io::Result<libc::sigset_t> {
    let mut signals = empty_signal_set()?;
    add_signal(&mut signals, libc::SIGINT)?;
    add_signal(&mut signals, libc::SIGTERM)?;

    let rc = unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &signals, std::ptr::null_mut()) };
    if rc != 0 {
        return Err(io::Error::from_raw_os_error(rc));
    }
    Ok(signals)
}

#[derive(Clone, Debug)]
struct ShutdownSignalState {
    inner: Arc<Mutex<ShutdownSignalInner>>,
}

#[derive(Debug, Default)]
struct ShutdownSignalInner {
    shutdown: Option<SessionShutdownHandle>,
    pending_signal: Option<libc::c_int>,
}

impl ShutdownSignalState {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ShutdownSignalInner::default())),
        }
    }

    fn install_shutdown(&self, shutdown: SessionShutdownHandle) {
        let mut inner = self.inner.lock().expect("shutdown signal state poisoned");
        if let Some(signal) = inner.pending_signal.take() {
            eprintln!(
                "forwarding pending {} to graceful ScreenFS shutdown",
                signal_name(signal)
            );
            shutdown.shutdown();
        }
        inner.shutdown = Some(shutdown);
    }

    fn handle_signal(&self, signal: libc::c_int) {
        let mut inner = self.inner.lock().expect("shutdown signal state poisoned");
        if let Some(shutdown) = &inner.shutdown {
            if shutdown.is_shutdown() {
                eprintln!(
                    "received {} after ScreenFS shutdown was already requested; still waiting for graceful teardown",
                    signal_name(signal)
                );
            } else {
                eprintln!(
                    "received {}, requesting graceful ScreenFS shutdown",
                    signal_name(signal)
                );
                shutdown.shutdown();
            }
        } else if inner.pending_signal.is_some() {
            eprintln!(
                "received {} before the FUSE session was ready and a startup shutdown is already pending; exiting startup",
                signal_name(signal)
            );
            std::process::exit(exit_code_for_signal(signal));
        } else {
            eprintln!(
                "received {} before the FUSE session was ready; shutdown will be requested after mount setup completes",
                signal_name(signal)
            );
            inner.pending_signal = Some(signal);
        }
    }
}

fn spawn_shutdown_signal_forwarder(
    signals: libc::sigset_t,
    state: ShutdownSignalState,
) -> io::Result<thread::JoinHandle<()>> {
    thread::Builder::new()
        .name("screenfs-signal".to_string())
        .spawn(move || {
            loop {
                let mut signal = 0;
                let rc = unsafe { libc::sigwait(&signals, &mut signal) };
                if rc == 0 {
                    state.handle_signal(signal);
                } else {
                    eprintln!(
                        "failed to wait for ScreenFS shutdown signal: {}",
                        io::Error::from_raw_os_error(rc)
                    );
                    break;
                }
            }
        })
}

fn empty_signal_set() -> io::Result<libc::sigset_t> {
    let mut signals = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
    let rc = unsafe { libc::sigemptyset(signals.as_mut_ptr()) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { signals.assume_init() })
}

fn add_signal(signals: &mut libc::sigset_t, signal: libc::c_int) -> io::Result<()> {
    let rc = unsafe { libc::sigaddset(signals, signal) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn signal_name(signal: libc::c_int) -> &'static str {
    match signal {
        libc::SIGINT => "SIGINT",
        libc::SIGTERM => "SIGTERM",
        _ => "unknown signal",
    }
}

fn exit_code_for_signal(signal: libc::c_int) -> i32 {
    128 + signal
}

fn normalize_mount_root_for_mountinfo(mount_root: &Path) -> PathBuf {
    mount_root
        .canonicalize()
        .unwrap_or_else(|_| mount_root.to_path_buf())
}

fn report_stale_mount_guidance(mount_root: &Path) -> io::Result<()> {
    if mountinfo_contains_mount_point(mount_root)? {
        let quoted_mount_root = shell_quote_path(mount_root);
        eprintln!(
            "ScreenFS shutdown returned but {} still appears mounted; retrying normal cleanup with: fusermount3 -u {}",
            mount_root.display(),
            quoted_mount_root
        );
        match fusermount_unmount(mount_root) {
            Ok(()) => {
                eprintln!(
                    "normal fusermount3 -u cleanup succeeded for {}",
                    mount_root.display()
                );
            }
            Err(error) => {
                eprintln!(
                    "normal fusermount3 -u cleanup failed for {}: {error}. Lazy unmount is not automatic. If the mount is stale and no process should keep it busy, clean it up manually with: fusermount3 -uz {}",
                    mount_root.display(),
                    quoted_mount_root
                );
            }
        }
    }
    Ok(())
}

fn fusermount_unmount(mount_root: &Path) -> io::Result<()> {
    let status = Command::new("fusermount3")
        .arg("-u")
        .arg(mount_root)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "fusermount3 -u exited with {status}"
        )))
    }
}

fn shell_quote_path(path: &Path) -> String {
    let path = path.as_os_str().to_string_lossy();
    format!("'{}'", path.replace('\'', "'\\''"))
}

fn mountinfo_contains_mount_point(mount_root: &Path) -> io::Result<bool> {
    let mountinfo = std::fs::read_to_string("/proc/self/mountinfo")?;
    Ok(mountinfo_contains_mount_point_in(&mountinfo, mount_root))
}

fn mountinfo_contains_mount_point_in(mountinfo: &str, mount_root: &Path) -> bool {
    mountinfo.lines().any(|line| {
        let Some((mount_fields, fs_fields)) = line.split_once(" - ") else {
            return false;
        };
        let is_screenfs_fuse = {
            let mut fields = fs_fields.split_whitespace();
            matches!(fields.next(), Some("fuse.screenfs"))
                && matches!(fields.next(), Some("screenfs"))
        };
        is_screenfs_fuse
            && mount_fields
                .split_whitespace()
                .nth(4)
                .map(decode_mountinfo_path)
                .is_some_and(|mount_point| PathBuf::from(mount_point) == mount_root)
    })
}

fn decode_mountinfo_path(encoded: &str) -> String {
    let mut decoded = String::with_capacity(encoded.len());
    let mut chars = encoded.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let mut octal = String::new();
            for _ in 0..3 {
                match chars.peek().copied() {
                    Some(next) if next.is_ascii_digit() => {
                        octal.push(next);
                        chars.next();
                    }
                    _ => break,
                }
            }
            if octal.len() == 3 {
                if let Ok(value) = u8::from_str_radix(&octal, 8) {
                    decoded.push(value as char);
                } else {
                    decoded.push('\\');
                    decoded.push_str(&octal);
                }
            } else {
                decoded.push('\\');
                decoded.push_str(&octal);
            }
        } else {
            decoded.push(ch);
        }
    }
    decoded
}

/// Build the stable mount options ScreenFS passes to `fusermount3`.
///
/// Keep the async scope transport-only here: these options configure the FUSE
/// mount itself, while `fractal-fuse = 0.4.0` enforces `FUSE_OVER_IO_URING`
/// during `Session::run`'s `FUSE_INIT` negotiation. Do not use this helper to
/// opt backing host filesystem I/O into `io_uring`.
fn screenfs_mount_options(experimental_writeback_cache: bool) -> MountOptions {
    let opts = MountOptions::new()
        .fs_name("screenfs")
        // Handler-level readonly is the source of truth. Do not set mount-level ro
        // until mount smoke proves it preserves hidden-before-EROFS semantics.
        .read_only(false)
        .force_readdir_plus(true)
        .default_permissions(false)
        .allow_other(false);
    if experimental_writeback_cache {
        opts.write_back(true)
    } else {
        opts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screenfs_mount_options_preserve_current_mount_policy() {
        let opts = screenfs_mount_options(false);

        assert_eq!(opts.fs_name.as_deref(), Some("screenfs"));
        assert!(!opts.read_only);
        assert!(opts.force_readdir_plus);
        assert!(!opts.default_permissions);
        assert!(!opts.allow_other);
        assert!(!opts.allow_root);
        assert!(!opts.dont_mask);
        assert!(!opts.no_open_support);
        assert!(!opts.no_open_dir_support);
        assert!(!opts.handle_killpriv);
        assert!(!opts.write_back);
        assert!(!opts.passthrough);
        assert!(!opts.posix_locks);
        assert!(!opts.flock_locks);
        assert!(opts.uid.is_none());
        assert!(opts.gid.is_none());
        assert!(opts.rootmode.is_none());
        assert!(opts.custom_options.is_none());
    }

    #[test]
    fn screenfs_mount_options_enable_writeback_only_for_experiment() {
        let default_opts = screenfs_mount_options(false);
        let experimental_opts = screenfs_mount_options(true);

        assert!(!default_opts.write_back);
        assert!(experimental_opts.write_back);
        assert_eq!(experimental_opts.fs_name.as_deref(), Some("screenfs"));
        assert!(!experimental_opts.read_only);
        assert!(experimental_opts.force_readdir_plus);
    }

    #[test]
    fn decode_mountinfo_path_decodes_octal_escapes() {
        assert_eq!(
            decode_mountinfo_path(r"/tmp/screenfs\040root"),
            "/tmp/screenfs root"
        );
        assert_eq!(
            decode_mountinfo_path(r"/tmp/screenfs\011root"),
            "/tmp/screenfs\troot"
        );
        assert_eq!(
            decode_mountinfo_path(r"/tmp/screenfs\root"),
            r"/tmp/screenfs\root"
        );
    }

    #[test]
    fn mountinfo_contains_mount_point_matches_decoded_field() {
        let mountinfo = "24 18 0:21 / / rw,relatime - ext4 /dev/root rw\n\
                         41 24 0:32 / /tmp/screenfs\\040root rw,relatime - ext4 /dev/sda rw\n\
                         42 24 0:33 / /tmp/screenfs\\040root rw,nosuid,nodev - fuse.screenfs screenfs rw\n";

        assert!(mountinfo_contains_mount_point_in(
            mountinfo,
            Path::new("/tmp/screenfs root")
        ));
        assert!(!mountinfo_contains_mount_point_in(
            mountinfo,
            Path::new("/tmp/other")
        ));
    }

    #[test]
    fn shell_quote_path_quotes_spaces_and_single_quotes() {
        assert_eq!(
            shell_quote_path(Path::new("/tmp/screenfs root")),
            "'/tmp/screenfs root'"
        );
        assert_eq!(
            shell_quote_path(Path::new("/tmp/o'clock")),
            "'/tmp/o'\\''clock'"
        );
    }
}
