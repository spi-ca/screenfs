//! Shared test harness for mount-free filesystem behavior tests.

use super::*;
use crate::cli::{CliArgs, LaunchArgs, MutabilityDefault};
use crate::config::RuntimeConfig;
use crate::fs::backing::cstring_path;
use crate::path::{ProcessEnvGuard, VirtualPath};
use fractal_fuse::abi::FUSE_ROOT_ID;
use std::fs::{File, OpenOptions};
use std::future::Future;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU64, Ordering},
};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{SystemTime, UNIX_EPOCH};

// Subtest modules keep large behavior areas readable.
#[path = "tests/data_mutations.rs"]
mod data_mutations;
#[path = "tests/mutability.rs"]
mod mutability;
#[path = "tests/perf.rs"]
#[cfg(feature = "perf-counters")]
mod perf;
#[path = "tests/state_cache.rs"]
mod state_cache;
#[path = "tests/symlinks_access_create.rs"]
mod symlinks_access_create;
#[path = "tests/visibility.rs"]
mod visibility;

fn fs_for_policy(
    source: &Path,
    visibility_hidden_rules: Vec<String>,
    mutability_readonly_rules: Vec<String>,
    mutability_default: Option<MutabilityDefault>,
    mutability_writable_rules: Vec<String>,
) -> ScreenFs {
    let mount = source.join("mount");
    std::fs::create_dir_all(&mount).unwrap();
    let _env = ProcessEnvGuard::new(source, None);
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.to_path_buf(),
            mount_root: mount,
            visibility_hidden_rules,
            visibility_visible_rules: Vec::new(),
            mutability_readonly_rules,
            mutability_writable_rules,
        },
        config_path: None,
        visibility_default: None,
        mutability_default,
    })
    .unwrap();
    ScreenFs::new(cfg)
}

fn fs_for(
    source: &Path,
    visibility_hidden_rules: Vec<String>,
    mutability_readonly_rules: Vec<String>,
) -> ScreenFs {
    fs_for_policy(
        source,
        visibility_hidden_rules,
        mutability_readonly_rules,
        None,
        Vec::new(),
    )
}

fn fs_for_external_mount(source: &Path, mount: &Path) -> ScreenFs {
    std::fs::create_dir_all(mount).unwrap();
    let _env = ProcessEnvGuard::new(source, None);
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.to_path_buf(),
            mount_root: mount.to_path_buf(),
            visibility_hidden_rules: Vec::new(),
            visibility_visible_rules: Vec::new(),
            mutability_readonly_rules: Vec::new(),
            mutability_writable_rules: Vec::new(),
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
    })
    .unwrap();
    ScreenFs::new(cfg)
}

fn fs_for_root_readonly(source: &Path, visibility_hidden_rules: Vec<String>) -> ScreenFs {
    fs_for_policy(
        source,
        visibility_hidden_rules,
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        Vec::new(),
    )
}

fn fs_for_axes(
    source: &Path,
    visibility_default: Option<crate::cli::VisibilityDefault>,
    hidden: Vec<String>,
    visible: Vec<String>,
    mutability_default: Option<MutabilityDefault>,
    readonly: Vec<String>,
    writable: Vec<String>,
) -> ScreenFs {
    let mount = source.join("mount");
    std::fs::create_dir_all(&mount).unwrap();
    let _env = ProcessEnvGuard::new(source, None);
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.to_path_buf(),
            mount_root: mount,
            visibility_hidden_rules: hidden,
            visibility_visible_rules: visible,
            mutability_readonly_rules: readonly,
            mutability_writable_rules: writable,
        },
        config_path: None,
        visibility_default,
        mutability_default,
    })
    .unwrap();
    ScreenFs::new(cfg)
}

#[cfg(feature = "perf-counters")]
fn fs_for_perf(source: &Path) -> ScreenFs {
    let mount = source.join("mount");
    std::fs::create_dir_all(&mount).unwrap();
    let _env = ProcessEnvGuard::new(source, None);
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.to_path_buf(),
            mount_root: mount,
            visibility_hidden_rules: Vec::new(),
            visibility_visible_rules: Vec::new(),
            mutability_readonly_rules: Vec::new(),
            mutability_writable_rules: Vec::new(),
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
    })
    .unwrap();
    ScreenFs::new(cfg)
}

fn vpath(path: &str) -> VirtualPath {
    VirtualPath::new(path)
}

fn tracked_inode(fs: &ScreenFs, path: &str) -> u64 {
    fs.state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(vpath(path))
}

fn insert_tracked_file_handle(fs: &ScreenFs, inode: u64, path: &str, file: File) -> u64 {
    fs.state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(inode, vpath(path), file)
}

fn lookup_child_inode(fs: &ScreenFs, parent: u64, name: &str) -> u64 {
    block_on(fs.lookup(dummy_req(), parent, OsStr::new(name)))
        .unwrap()
        .attr
        .ino
}

fn lookup_root_inode(fs: &ScreenFs, name: &str) -> u64 {
    lookup_child_inode(fs, FUSE_ROOT_ID, name)
}

fn open_directory_handle(fs: &ScreenFs, inode: u64) -> u64 {
    block_on(fs.opendir(dummy_req(), inode, libc::O_RDONLY as u32))
        .unwrap()
        .fh
}

fn root_listing_names(fs: &ScreenFs) -> Vec<String> {
    let fh = open_directory_handle(fs, FUSE_ROOT_ID);
    let names = block_on(fs.readdirplus(dummy_req(), FUSE_ROOT_ID, fh, 0, 4096))
        .unwrap()
        .into_iter()
        .map(|entry| String::from_utf8(entry.name).unwrap())
        .collect();
    block_on(fs.releasedir(dummy_req(), FUSE_ROOT_ID, fh, 0)).unwrap();
    names
}

fn assert_root_recreation_replaces_inode(
    fs: &ScreenFs,
    name: &str,
    remove: impl FnOnce(&ScreenFs),
    recreate: impl FnOnce(&ScreenFs) -> u64,
) {
    let old_inode = lookup_root_inode(fs, name);
    let root_fh = open_directory_handle(fs, FUSE_ROOT_ID);
    remove(fs);
    assert_readdir_handle_invalidated(fs, FUSE_ROOT_ID, root_fh);

    let root_fh = open_directory_handle(fs, FUSE_ROOT_ID);
    let new_inode = recreate(fs);
    assert_readdir_handle_invalidated(fs, FUSE_ROOT_ID, root_fh);
    assert_ne!(new_inode, old_inode);
}

fn assert_readdir_handle_invalidated(fs: &ScreenFs, inode: u64, fh: u64) {
    assert_eq!(
        block_on(fs.readdir(dummy_req(), inode, fh, 0, 4096)).unwrap_err(),
        ENOENT
    );
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut future = Box::pin(future);
    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("test future unexpectedly pending"),
    }
}

fn compio_block_on<F: Future>(future: F) -> F::Output {
    compio_runtime::Runtime::new()
        .expect("compio runtime must start for tests")
        .block_on(future)
}

fn waking_block_on<F: Future>(future: F) -> F::Output {
    struct ThreadWaker(std::thread::Thread);
    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.0.unpark();
        }
    }

    let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        if let Poll::Ready(value) = Pin::new(&mut future).poll(&mut cx) {
            return value;
        }
        std::thread::park();
    }
}

fn test_dir(label: &str) -> std::path::PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let mut dir = test_scratch_root();
    std::fs::create_dir_all(&dir).unwrap();
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let pid = std::process::id();
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    dir.push(format!("screenfs-{label}-{pid}-{id}-{counter}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn test_scratch_root() -> PathBuf {
    if let Some(root) = std::env::var_os("SCREENFS_TEST_TMPDIR").map(PathBuf::from) {
        return root;
    }
    static SCRATCH_ROOT: OnceLock<PathBuf> = OnceLock::new();
    SCRATCH_ROOT
        .get_or_init(|| {
            let shm = Path::new("/dev/shm").join("screenfs-tests");
            if scratch_root_supports_open_fd_lifetime(&shm) {
                shm
            } else {
                PathBuf::from("/tmp/screenfs-tests")
            }
        })
        .clone()
}

fn scratch_root_supports_open_fd_lifetime(root: &Path) -> bool {
    let probe = root.join("probe-open-fd-lifetime");
    let original = probe.join("file");
    let renamed = probe.join("renamed");
    let result = (|| -> std::io::Result<bool> {
        std::fs::create_dir_all(&probe)?;
        std::fs::write(&original, b"ok")?;
        let file = OpenOptions::new().read(true).write(true).open(&original)?;
        std::fs::rename(&original, &renamed)?;
        let mut buf = [0_u8; 2];
        if file.read_at(&mut buf, 0)? != 2 || buf != *b"ok" {
            return Ok(false);
        }
        std::fs::remove_file(&renamed)?;
        if file.read_at(&mut buf, 0)? != 2 || buf != *b"ok" {
            return Ok(false);
        }
        Ok(true)
    })();
    let _ = std::fs::remove_dir_all(&probe);
    result.unwrap_or(false)
}

fn dummy_req() -> Request {
    Request {
        unique: 1,
        uid: 0,
        gid: 0,
        pid: 0,
    }
}

fn host_statfs(path: &Path) -> ReplyStatfs {
    let c_path = cstring_path(path).unwrap();
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::zeroed();
    let result = unsafe { libc::statvfs(c_path.as_ptr(), stats.as_mut_ptr()) };
    assert_eq!(result, 0, "statvfs failed for {}", path.display());
    let stats = unsafe { stats.assume_init() };
    let bsize = u32::try_from(stats.f_bsize)
        .ok()
        .filter(|value| *value != 0)
        .unwrap_or(512);
    let frsize = u32::try_from(stats.f_frsize)
        .ok()
        .filter(|value| *value != 0)
        .unwrap_or(bsize);
    let namelen = u32::try_from(stats.f_namemax)
        .ok()
        .filter(|value| *value != 0)
        .unwrap_or(255);
    ReplyStatfs {
        blocks: stats.f_blocks,
        bfree: stats.f_bfree,
        bavail: stats.f_bavail,
        files: stats.f_files,
        ffree: stats.f_ffree,
        bsize,
        namelen,
        frsize,
    }
}
