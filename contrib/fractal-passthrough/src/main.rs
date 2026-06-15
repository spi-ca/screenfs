use std::collections::HashMap;
use std::ffi::{CString, OsStr};
use std::fs::{self, File, OpenOptions};
use std::os::fd::FromRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{FileExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use fractal_fuse::types::{FileAttr, FileType, Timestamp};
use fractal_fuse::{
    DirectoryEntry, ENOENT, Filesystem, FsResult, MountOptions, ReplyAttr, ReplyCreate, ReplyEntry,
    ReplyOpen, ReplyStatfs, Request, Session, SetAttr,
};

const ROOT: u64 = 1;

struct Ptfs {
    root: PathBuf,
    paths: RwLock<HashMap<u64, PathBuf>>,
    files: RwLock<HashMap<u64, File>>,
    next_fh: AtomicU64,
}

impl Ptfs {
    fn new(root: PathBuf) -> Self {
        let mut paths = HashMap::new();
        paths.insert(ROOT, PathBuf::from("/"));
        Self {
            root,
            paths: RwLock::new(paths),
            files: RwLock::new(HashMap::new()),
            next_fh: AtomicU64::new(2),
        }
    }
    fn host(&self, v: &Path) -> PathBuf {
        if v == Path::new("/") {
            self.root.clone()
        } else {
            self.root.join(v.strip_prefix("/").unwrap_or(v))
        }
    }
    fn path_for(&self, ino: u64) -> Result<PathBuf, i32> {
        self.paths.read().unwrap().get(&ino).cloned().ok_or(ENOENT)
    }
    fn child(&self, parent: u64, name: &OsStr) -> Result<PathBuf, i32> {
        let mut p = self.path_for(parent)?;
        p.push(name);
        Ok(p)
    }
    fn attr_for(&self, v: &Path, ino_override: Option<u64>) -> Result<FileAttr, i32> {
        let md = fs::symlink_metadata(self.host(v)).map_err(errno)?;
        Ok(attr_from(
            &md,
            ino_override.unwrap_or_else(|| if v == Path::new("/") { ROOT } else { md.ino() }),
        ))
    }
    fn track(&self, v: PathBuf) -> Result<FileAttr, i32> {
        let attr = self.attr_for(&v, None)?;
        self.paths.write().unwrap().insert(attr.ino, v);
        Ok(attr)
    }
    fn file(&self, fh: u64) -> Result<File, i32> {
        self.files
            .read()
            .unwrap()
            .get(&fh)
            .ok_or(ENOENT)?
            .try_clone()
            .map_err(errno)
    }

    fn parent_ino(&self, path: &Path) -> Result<u64, i32> {
        if path == Path::new("/") {
            return Ok(ROOT);
        }
        let parent = path.parent().unwrap_or(Path::new("/"));
        Ok(self.attr_for(parent, None)?.ino)
    }
}

fn errno(e: std::io::Error) -> i32 {
    e.raw_os_error().unwrap_or(libc::EIO)
}

fn ts(sec: i64, nsec: i64) -> Timestamp {
    Timestamp {
        sec: sec as u64,
        nsec: nsec as u32,
    }
}

fn attr_from(md: &fs::Metadata, ino: u64) -> FileAttr {
    FileAttr {
        ino,
        size: md.size(),
        blocks: md.blocks(),
        atime: ts(md.atime(), md.atime_nsec()),
        mtime: ts(md.mtime(), md.mtime_nsec()),
        ctime: ts(md.ctime(), md.ctime_nsec()),
        mode: md.mode(),
        nlink: md.nlink() as u32,
        uid: md.uid(),
        gid: md.gid(),
        rdev: md.rdev() as u32,
        blksize: md.blksize() as u32,
    }
}

fn c_path(path: &Path) -> Result<CString, i32> {
    CString::new(path.as_os_str().as_bytes()).map_err(|_| libc::EINVAL)
}
fn file_type(md: &fs::Metadata) -> FileType {
    let mode = md.mode() & libc::S_IFMT;
    if mode == libc::S_IFDIR {
        FileType::Directory
    } else if mode == libc::S_IFLNK {
        FileType::Symlink
    } else {
        FileType::RegularFile
    }
}

impl Filesystem for Ptfs {
    async fn lookup(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<ReplyEntry> {
        let path = self.child(parent, name)?;
        let attr = self.track(path)?;
        Ok(ReplyEntry {
            ttl: Duration::ZERO,
            attr,
            generation: 0,
        })
    }
    async fn getattr(
        &self,
        _req: Request,
        inode: u64,
        _fh: Option<u64>,
        _flags: u32,
    ) -> FsResult<ReplyAttr> {
        let path = self.path_for(inode)?;
        Ok(ReplyAttr {
            ttl: Duration::ZERO,
            attr: self.attr_for(&path, Some(inode))?,
        })
    }
    async fn open(&self, _req: Request, inode: u64, flags: u32) -> FsResult<ReplyOpen> {
        let path = self.host(&self.path_for(inode)?);
        let c = c_path(&path)?;
        let fd = unsafe { libc::open(c.as_ptr(), flags as libc::c_int, 0o666) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO));
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let fh = self.next_fh.fetch_add(1, Ordering::Relaxed);
        self.files.write().unwrap().insert(fh, file);
        Ok(ReplyOpen {
            fh,
            flags: 0,
            backing_id: 0,
        })
    }
    async fn create(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32,
    ) -> FsResult<ReplyCreate> {
        let path = self.child(parent, name)?;
        let host = self.host(&path);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(mode)
            .custom_flags((flags as i32) & !libc::O_CREAT)
            .open(host)
            .map_err(errno)?;
        let attr = self.track(path)?;
        let fh = self.next_fh.fetch_add(1, Ordering::Relaxed);
        self.files.write().unwrap().insert(fh, file);
        Ok(ReplyCreate {
            ttl: Duration::ZERO,
            attr,
            generation: 0,
            fh,
            flags: 0,
        })
    }
    async fn read(
        &self,
        _req: Request,
        _inode: u64,
        fh: u64,
        offset: u64,
        buf: &mut [u8],
    ) -> FsResult<usize> {
        self.file(fh)?.read_at(buf, offset).map_err(errno)
    }
    async fn write(
        &self,
        _req: Request,
        _inode: u64,
        fh: u64,
        offset: u64,
        data: &[u8],
        _write_flags: u32,
        _flags: u32,
    ) -> FsResult<usize> {
        self.file(fh)?.write_at(data, offset).map_err(errno)
    }
    async fn fsync(&self, _req: Request, _inode: u64, fh: u64, datasync: bool) -> FsResult<()> {
        let file = self.file(fh)?;
        let r = if datasync {
            file.sync_data()
        } else {
            file.sync_all()
        };
        r.map_err(errno)
    }
    async fn flush(&self, _req: Request, _inode: u64, _fh: u64, _lock_owner: u64) -> FsResult<()> {
        Ok(())
    }
    async fn release(
        &self,
        _req: Request,
        _inode: u64,
        fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
        _flock_release: bool,
    ) -> FsResult<()> {
        self.files.write().unwrap().remove(&fh);
        Ok(())
    }
    async fn setattr(
        &self,
        _req: Request,
        inode: u64,
        fh: Option<u64>,
        set_attr: SetAttr,
    ) -> FsResult<ReplyAttr> {
        if let Some(size) = set_attr.size {
            if let Some(fh) = fh.or(set_attr.fh) {
                self.file(fh)?.set_len(size).map_err(errno)?;
            } else {
                fs::OpenOptions::new()
                    .write(true)
                    .open(self.host(&self.path_for(inode)?))
                    .and_then(|f| f.set_len(size))
                    .map_err(errno)?;
            }
        }
        let path = self.path_for(inode)?;
        Ok(ReplyAttr {
            ttl: Duration::ZERO,
            attr: self.attr_for(&path, Some(inode))?,
        })
    }
    async fn unlink(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<()> {
        fs::remove_file(self.host(&self.child(parent, name)?)).map_err(errno)
    }
    async fn readdir(
        &self,
        _req: Request,
        inode: u64,
        _fh: u64,
        offset: u64,
        _size: u32,
    ) -> FsResult<Vec<DirectoryEntry>> {
        let path = self.path_for(inode)?;
        let mut out = Vec::new();
        if offset == 0 {
            out.push(DirectoryEntry {
                ino: inode,
                offset: 1,
                kind: FileType::Directory,
                name: b".".to_vec(),
            });
        }
        if offset <= 1 {
            out.push(DirectoryEntry {
                ino: self.parent_ino(&path)?,
                offset: 2,
                kind: FileType::Directory,
                name: b"..".to_vec(),
            });
        }
        let start = offset.saturating_sub(2) as usize;
        for (idx, ent) in fs::read_dir(self.host(&path))
            .map_err(errno)?
            .skip(start)
            .enumerate()
        {
            let entry_index = start + idx;
            let ent = ent.map_err(errno)?;
            let mut child = path.clone();
            child.push(ent.file_name());
            let attr = self.track(child)?;
            let md = ent.metadata().map_err(errno)?;
            out.push(DirectoryEntry {
                ino: attr.ino,
                offset: entry_index as u64 + 3,
                kind: file_type(&md),
                name: ent.file_name().as_bytes().to_vec(),
            });
        }
        Ok(out)
    }
    async fn statfs(&self, _req: Request, _inode: u64) -> FsResult<ReplyStatfs> {
        let mut st = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        let c = c_path(&self.root)?;
        let rc = unsafe { libc::statvfs(c.as_ptr(), st.as_mut_ptr()) };
        if rc != 0 {
            return Err(std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO));
        }
        let st = unsafe { st.assume_init() };
        Ok(ReplyStatfs {
            blocks: st.f_blocks,
            bfree: st.f_bfree,
            bavail: st.f_bavail,
            files: st.f_files,
            ffree: st.f_ffree,
            bsize: st.f_bsize as u32,
            namelen: st.f_namemax as u32,
            frsize: st.f_frsize as u32,
        })
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("usage: fractal_passthrough SOURCE MOUNT");
        std::process::exit(2);
    }
    if unsafe { libc::geteuid() } == 0 {
        eprintln!("fractal_passthrough is a non-root benchmark helper; refusing to run as root");
        std::process::exit(2);
    }
    let root = fs::canonicalize(&args[1])?;
    let mount = PathBuf::from(&args[2]);
    if fs::canonicalize(&mount).is_ok_and(|mount| mount.starts_with(&root)) {
        eprintln!("MOUNT must not be inside SOURCE");
        std::process::exit(2);
    }
    let opts = MountOptions::new()
        .fs_name("fractal-passthrough")
        .force_readdir_plus(false)
        .default_permissions(false)
        .allow_other(false);
    eprintln!(
        "mounting fractal passthrough: source={} mount={} io_uring=required",
        root.display(),
        mount.display()
    );
    Session::new(mount, opts)?.run(Ptfs::new(root))
}
