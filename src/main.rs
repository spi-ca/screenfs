//! ScreenFS binary entrypoint.
//!
//! The binary validates launch inputs, builds runtime config, sets stable FUSE
//! mount options, and runs the `fractal-fuse` session with no fallback mount path.

use fractal_fuse::{MountOptions, Session};
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

    let cfg = RuntimeConfig::from_launch(args)
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;

    eprintln!(
        "mounting screenfs: source={} mount={} visibility-default={} visibility-source={} hidden-rules={} visible-rules={} mutability-default={} mutability-source={} readonly-rules={} writable-rules={} io_uring=required",
        cfg.source_root.display(),
        cfg.mount_root.display(),
        cfg.visibility_default().as_str(),
        cfg.visibility_source().as_str(),
        cfg.hidden_rule_count(),
        cfg.visible_rule_count(),
        cfg.mutability_default().as_str(),
        cfg.mutability_source().as_str(),
        cfg.readonly_rule_count(),
        cfg.writable_rule_count()
    );

    let opts = screenfs_mount_options();

    let mount_root = cfg.mount_root.clone();
    // `fractal-fuse = 0.4.0` enforces `FUSE_OVER_IO_URING` during
    // `Session::run`'s `FUSE_INIT` negotiation. Preserve the current
    // fail-fast contract by surfacing that error directly with no fallback.
    Session::new(mount_root, opts)?.run(ScreenFs::new(cfg))
}

/// Build the stable mount options ScreenFS passes to `fusermount3`.
///
/// Keep the async scope transport-only here: these options configure the FUSE
/// mount itself, while `fractal-fuse = 0.4.0` enforces `FUSE_OVER_IO_URING`
/// during `Session::run`'s `FUSE_INIT` negotiation. Do not use this helper to
/// opt backing host filesystem I/O into `io_uring`.
fn screenfs_mount_options() -> MountOptions {
    MountOptions::new()
        .fs_name("screenfs")
        // Handler-level readonly is the source of truth. Do not set mount-level ro
        // until mount smoke proves it preserves hidden-before-EROFS semantics.
        .read_only(false)
        .force_readdir_plus(true)
        .default_permissions(false)
        .allow_other(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screenfs_mount_options_preserve_current_mount_policy() {
        let opts = screenfs_mount_options();

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
}
