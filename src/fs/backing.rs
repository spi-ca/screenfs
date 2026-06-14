use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
#[cfg(feature = "perf-counters")]
use std::time::Instant;

use fractal_fuse::{FileAttr, FileType, ReplyStatfs, ReplyXattr, SetAttr, SetAttrTime, Timestamp};

use crate::errors::errno_from_io;
use crate::path::VirtualPath;

use super::ScreenFs;

const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
const RESOLVE_IN_ROOT: u64 = 0x10;

#[repr(C)]
struct OpenHow {
    flags: u64,
    mode: u64,
    resolve: u64,
}

impl ScreenFs {
    pub(super) fn open_confined(
        &self,
        path: &VirtualPath,
        flags: i32,
        mode: Option<u32>,
    ) -> Result<File, i32> {
        #[cfg(not(feature = "perf-counters"))]
        {
            open_beneath_source_root(&self.source_root, path, flags, mode)
        }
        #[cfg(feature = "perf-counters")]
        {
            let start = Instant::now();
            let result = open_beneath_source_root(&self.source_root, path, flags, mode);
            self.perf.record_open_confined(start.elapsed());
            result
        }
    }

    pub(super) fn source_root_path(&self) -> Result<std::path::PathBuf, i32> {
        fd_path(&self.source_root)
    }
}

pub(super) fn cstring_path(path: &Path) -> Result<CString, i32> {
    CString::new(path.as_os_str().as_bytes()).map_err(|_| libc::EINVAL)
}

pub(super) fn sanitize_open_flags(flags: u32, creating: bool) -> i32 {
    let mut sanitized = (flags as i32) & libc::O_ACCMODE;
    for allowed in [
        libc::O_APPEND,
        libc::O_TRUNC,
        libc::O_EXCL,
        libc::O_CLOEXEC,
        libc::O_NOFOLLOW,
        libc::O_NONBLOCK,
        libc::O_SYNC,
        libc::O_DSYNC,
        libc::O_RSYNC,
        libc::O_DIRECT,
        libc::O_NOATIME,
    ] {
        sanitized |= (flags as i32) & allowed;
    }
    if creating {
        sanitized |= libc::O_CREAT;
    }
    sanitized
}

pub(super) fn open_child_at(
    parent_dir: &File,
    name: &CStr,
    flags: i32,
    mode: Option<u32>,
) -> Result<File, i32> {
    let fd = unsafe {
        libc::openat(
            parent_dir.as_raw_fd(),
            name.as_ptr(),
            flags | libc::O_CLOEXEC,
            mode.unwrap_or(0) as libc::mode_t,
        )
    };
    if fd < 0 {
        Err(errno_from_io(std::io::Error::last_os_error()))
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

pub(super) fn open_beneath_source_root(
    source_root: &File,
    path: &VirtualPath,
    flags: i32,
    mode: Option<u32>,
) -> Result<File, i32> {
    let relative = path.to_source_relative_path();
    let relative = if relative.as_os_str().is_empty() {
        Path::new(".")
    } else {
        relative.as_path()
    };
    let c_path = cstring_path(relative)?;
    let how = OpenHow {
        flags: (flags | libc::O_CLOEXEC) as u64,
        mode: mode.unwrap_or(0) as u64,
        resolve: RESOLVE_IN_ROOT | RESOLVE_NO_MAGICLINKS,
    };
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            source_root.as_raw_fd(),
            c_path.as_ptr(),
            &how,
            std::mem::size_of::<OpenHow>(),
        ) as libc::c_int
    };
    if fd < 0 {
        Err(normalize_confined_errno(errno_from_io(
            std::io::Error::last_os_error(),
        )))
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

pub(super) fn open_dir_handle(path: &Path) -> Result<File, i32> {
    let c_path = cstring_path(path)?;
    let fd = unsafe {
        libc::open(
            c_path.as_ptr(),
            libc::O_PATH | libc::O_DIRECTORY | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        Err(errno_from_io(std::io::Error::last_os_error()))
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

pub(super) fn faccessat2_empty(file: &File, mask: u32) -> Result<(), i32> {
    let empty = cstring_os(OsStr::new(""))?;
    let result = unsafe {
        libc::syscall(
            libc::SYS_faccessat2,
            file.as_raw_fd(),
            empty.as_ptr(),
            mask as libc::c_int,
            libc::AT_EMPTY_PATH,
        ) as libc::c_int
    };
    if result == 0 {
        Ok(())
    } else {
        Err(normalize_confined_errno(errno_from_io(
            std::io::Error::last_os_error(),
        )))
    }
}

fn normalize_confined_errno(errno: i32) -> i32 {
    match errno {
        libc::EXDEV => libc::ENOENT,
        other => other,
    }
}

pub(super) fn cstring_os(value: &OsStr) -> Result<CString, i32> {
    CString::new(value.as_bytes()).map_err(|_| libc::EINVAL)
}

pub(super) fn fd_path(file: &File) -> Result<std::path::PathBuf, i32> {
    std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd())).map_err(errno_from_io)
}

pub(super) fn source_root_statfs(source_root: &File) -> Result<ReplyStatfs, i32> {
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::zeroed();
    let result = unsafe { libc::fstatvfs(source_root.as_raw_fd(), stats.as_mut_ptr()) };
    if result != 0 {
        return Err(errno_from_io(std::io::Error::last_os_error()));
    }
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
    Ok(ReplyStatfs {
        blocks: stats.f_blocks,
        bfree: stats.f_bfree,
        bavail: stats.f_bavail,
        files: stats.f_files,
        ffree: stats.f_ffree,
        bsize,
        namelen,
        frsize,
    })
}

pub(super) fn apply_setattr(file: &File, set_attr: SetAttr) -> Result<(), i32> {
    let fd = file.as_raw_fd();
    if let Some(mode) = set_attr.mode {
        let result = unsafe { libc::fchmod(fd, (mode & 0o7777) as libc::mode_t) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
    }
    if set_attr.uid.is_some() || set_attr.gid.is_some() {
        let uid = set_attr.uid.unwrap_or(u32::MAX) as libc::uid_t;
        let gid = set_attr.gid.unwrap_or(u32::MAX) as libc::gid_t;
        let result = unsafe { libc::fchown(fd, uid, gid) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
    }
    if let Some(size) = set_attr.size {
        let result = unsafe { libc::ftruncate(fd, size as libc::off_t) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
    }
    if set_attr.atime.is_some() || set_attr.mtime.is_some() {
        let current = file.metadata().map_err(errno_from_io)?;
        let atime = set_attr.atime.unwrap_or_else(|| {
            SetAttrTime::Specific(Timestamp::new(
                current.atime() as u64,
                current.atime_nsec() as u32,
            ))
        });
        let mtime = set_attr.mtime.unwrap_or_else(|| {
            SetAttrTime::Specific(Timestamp::new(
                current.mtime() as u64,
                current.mtime_nsec() as u32,
            ))
        });
        let times = [timespec_from_setattr(atime), timespec_from_setattr(mtime)];
        let result = unsafe { libc::futimens(fd, times.as_ptr()) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}

fn timespec_from_setattr(value: SetAttrTime) -> libc::timespec {
    match value {
        SetAttrTime::Now => libc::timespec {
            tv_sec: 0,
            tv_nsec: libc::UTIME_NOW,
        },
        SetAttrTime::Specific(ts) => libc::timespec {
            tv_sec: ts.sec as libc::time_t,
            tv_nsec: ts.nsec as libc::c_long,
        },
    }
}

#[derive(Debug, Clone)]
pub(super) struct DirEntryInfo {
    pub(super) name: OsString,
    pub(super) child: VirtualPath,
    pub(super) attr: FileAttr,
    pub(super) kind: FileType,
    pub(super) is_dir: bool,
    pub(super) is_symlink: bool,
}

impl ScreenFs {
    pub(super) fn open_parent_dir(
        &self,
        path: &VirtualPath,
    ) -> Result<(VirtualPath, File, CString), i32> {
        let parent = Self::parent_path(path);
        let name = path.as_path().file_name().ok_or(libc::EINVAL)?;
        let parent_dir = self.open_confined(&parent, libc::O_PATH | libc::O_DIRECTORY, None)?;
        Ok((parent, parent_dir, cstring_os(name)?))
    }

    pub(super) fn stat_child_no_follow(
        &self,
        path: &VirtualPath,
        ino: u64,
    ) -> Result<FileAttr, i32> {
        if path.as_path() == Path::new("/") {
            let file = self.open_confined(path, libc::O_PATH | libc::O_DIRECTORY, None)?;
            return fstat_attr(&file, ino);
        }
        let (parent, parent_dir, name) = self.open_parent_dir(path)?;
        self.guard_opened_directory_target(&parent, &parent_dir, false)?;
        fstatat_attr(&parent_dir, &name, libc::AT_SYMLINK_NOFOLLOW, ino)
    }

    pub(super) fn readlink_child(&self, path: &VirtualPath) -> Result<OsString, i32> {
        let (parent, parent_dir, name) = self.open_parent_dir(path)?;
        self.guard_opened_directory_target(&parent, &parent_dir, false)?;
        readlinkat_os(&parent_dir, &name)
    }
}

pub(super) fn fstat_attr(file: &File, ino: u64) -> Result<FileAttr, i32> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    let result = unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) };
    if result == 0 {
        let stat = unsafe { stat.assume_init() };
        Ok(stat_to_attr(&stat, ino))
    } else {
        Err(errno_from_io(std::io::Error::last_os_error()))
    }
}

pub(super) fn fstatat_attr(
    parent_dir: &File,
    name: &CStr,
    flags: i32,
    ino: u64,
) -> Result<FileAttr, i32> {
    fstatat_attr_fd(parent_dir.as_raw_fd(), name, flags, ino)
}

fn fstatat_attr_fd(fd: i32, name: &CStr, flags: i32, ino: u64) -> Result<FileAttr, i32> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    let result = unsafe { libc::fstatat(fd, name.as_ptr(), stat.as_mut_ptr(), flags) };
    if result == 0 {
        let stat = unsafe { stat.assume_init() };
        Ok(stat_to_attr(&stat, ino))
    } else {
        Err(errno_from_io(std::io::Error::last_os_error()))
    }
}

pub(super) fn readlinkat_os(parent_dir: &File, name: &CStr) -> Result<OsString, i32> {
    let mut size = 256usize;
    loop {
        let mut data = vec![0_u8; size];
        let read = unsafe {
            libc::readlinkat(
                parent_dir.as_raw_fd(),
                name.as_ptr(),
                data.as_mut_ptr().cast(),
                data.len(),
            )
        };
        if read < 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        let read = read as usize;
        if read < data.len() {
            data.truncate(read);
            return Ok(OsString::from_vec(data));
        }
        size *= 2;
    }
}

pub(super) fn stat_to_attr(stat: &libc::stat, ino: u64) -> FileAttr {
    FileAttr {
        ino,
        size: stat.st_size as u64,
        blocks: stat.st_blocks as u64,
        atime: Timestamp::new(stat.st_atime as u64, stat.st_atime_nsec as u32),
        mtime: Timestamp::new(stat.st_mtime as u64, stat.st_mtime_nsec as u32),
        ctime: Timestamp::new(stat.st_ctime as u64, stat.st_ctime_nsec as u32),
        mode: stat.st_mode,
        nlink: stat.st_nlink as u32,
        uid: stat.st_uid,
        gid: stat.st_gid,
        rdev: stat.st_rdev as u32,
        blksize: stat.st_blksize as u32,
    }
}

pub(super) fn file_type_from_mode(mode: libc::mode_t) -> FileType {
    match mode & libc::S_IFMT {
        libc::S_IFDIR => FileType::Directory,
        libc::S_IFLNK => FileType::Symlink,
        libc::S_IFBLK => FileType::BlockDevice,
        libc::S_IFCHR => FileType::CharDevice,
        libc::S_IFIFO => FileType::NamedPipe,
        libc::S_IFSOCK => FileType::Socket,
        _ => FileType::RegularFile,
    }
}

pub(super) fn visit_dir_entries(
    dir: File,
    base: &VirtualPath,
    start_offset: u64,
    mut visit: impl FnMut(DirEntryInfo) -> Result<(), i32>,
) -> Result<(), i32> {
    let dir_fd = dir.into_raw_fd();
    let dirp = unsafe { libc::fdopendir(dir_fd) };
    if dirp.is_null() {
        let err = errno_from_io(std::io::Error::last_os_error());
        unsafe { libc::close(dir_fd) };
        return Err(err);
    }
    let mut seen = 0_u64;
    loop {
        errno_reset();
        let dent = unsafe { libc::readdir(dirp) };
        if dent.is_null() {
            let err = std::io::Error::last_os_error();
            unsafe { libc::closedir(dirp) };
            return if err.raw_os_error().unwrap_or(0) == 0 {
                Ok(())
            } else {
                Err(errno_from_io(err))
            };
        }
        let dent = unsafe { &*dent };
        let name = unsafe { CStr::from_ptr(dent.d_name.as_ptr()) };
        if name.to_bytes() == b"." || name.to_bytes() == b".." {
            continue;
        }
        let name_os = OsStr::from_bytes(name.to_bytes()).to_os_string();
        let child = base.join_child(&name_os);
        let attr = match fstatat_attr_fd(
            dir_fd,
            name,
            libc::AT_SYMLINK_NOFOLLOW,
            start_offset + seen + 1,
        ) {
            Ok(attr) => attr,
            Err(err) => {
                unsafe { libc::closedir(dirp) };
                return Err(err);
            }
        };
        seen += 1;
        let kind = file_type_from_mode(attr.mode);
        if let Err(err) = visit(DirEntryInfo {
            name: name_os,
            child,
            is_dir: matches!(kind, FileType::Directory),
            is_symlink: matches!(kind, FileType::Symlink),
            kind,
            attr,
        }) {
            unsafe { libc::closedir(dirp) };
            return Err(err);
        }
    }
}

fn errno_reset() {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe {
        *libc::__errno_location() = 0;
    }
}

pub(super) fn read_xattr_reply(
    size: u32,
    probe_len: impl FnOnce() -> Result<usize, i32>,
    read_value: impl FnOnce(&mut [u8]) -> Result<usize, i32>,
) -> Result<ReplyXattr, i32> {
    let needed = probe_len()?;
    if size == 0 {
        return Ok(ReplyXattr::Size(needed as u32));
    }
    let mut data = vec![0_u8; needed];
    let read = read_value(&mut data)?;
    data.truncate(read);
    Ok(ReplyXattr::Data(data))
}
