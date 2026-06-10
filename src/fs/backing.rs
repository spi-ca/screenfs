use std::ffi::{CString, OsStr};
use std::fs::{self, File, OpenOptions};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::Path;

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
    pub(super) fn host_path(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<std::path::PathBuf, i32> {
        path.resolve_host_path(&self.cfg.source_root, follow_final_symlink)
            .map_err(errno_from_io)
    }

    pub(super) fn open_confined(
        &self,
        path: &VirtualPath,
        flags: i32,
        mode: Option<u32>,
    ) -> Result<File, i32> {
        open_beneath_source_root(&self.cfg.source_root, path, flags, mode)
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

pub(super) fn open_beneath_source_root(
    source_root: &Path,
    path: &VirtualPath,
    flags: i32,
    mode: Option<u32>,
) -> Result<File, i32> {
    let root = open_dir_handle(source_root)?;
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
            root.as_raw_fd(),
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

fn open_dir_handle(path: &Path) -> Result<File, i32> {
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

pub(super) fn source_root_statfs(source_root: &Path) -> Result<ReplyStatfs, i32> {
    let c_path = cstring_path(source_root)?;
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::zeroed();
    let result = unsafe { libc::statvfs(c_path.as_ptr(), stats.as_mut_ptr()) };
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

pub(super) fn apply_setattr(path: &Path, set_attr: SetAttr) -> Result<(), i32> {
    if let Some(mode) = set_attr.mode {
        fs::set_permissions(path, fs::Permissions::from_mode(mode & 0o7777))
            .map_err(errno_from_io)?;
    }
    if set_attr.uid.is_some() || set_attr.gid.is_some() {
        let c_path = cstring_path(path)?;
        let uid = set_attr.uid.unwrap_or(u32::MAX) as libc::uid_t;
        let gid = set_attr.gid.unwrap_or(u32::MAX) as libc::gid_t;
        let result = unsafe { libc::chown(c_path.as_ptr(), uid, gid) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
    }
    if let Some(size) = set_attr.size {
        let file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(errno_from_io)?;
        file.set_len(size).map_err(errno_from_io)?;
    }
    if set_attr.atime.is_some() || set_attr.mtime.is_some() {
        let current = fs::metadata(path).map_err(errno_from_io)?;
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
        let c_path = cstring_path(path)?;
        let result = unsafe { libc::utimensat(libc::AT_FDCWD, c_path.as_ptr(), times.as_ptr(), 0) };
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

pub(super) fn metadata_to_attr(metadata: &fs::Metadata, ino: u64) -> FileAttr {
    FileAttr {
        ino,
        size: metadata.size(),
        blocks: metadata.blocks(),
        atime: Timestamp::new(metadata.atime() as u64, metadata.atime_nsec() as u32),
        mtime: Timestamp::new(metadata.mtime() as u64, metadata.mtime_nsec() as u32),
        ctime: Timestamp::new(metadata.ctime() as u64, metadata.ctime_nsec() as u32),
        mode: metadata.mode(),
        nlink: metadata.nlink() as u32,
        uid: metadata.uid(),
        gid: metadata.gid(),
        rdev: metadata.rdev() as u32,
        blksize: metadata.blksize() as u32,
    }
}

pub(super) fn file_type_from_metadata(metadata: &fs::Metadata) -> FileType {
    let ft = metadata.file_type();
    if ft.is_dir() {
        FileType::Directory
    } else if ft.is_symlink() {
        FileType::Symlink
    } else if ft.is_block_device() {
        FileType::BlockDevice
    } else if ft.is_char_device() {
        FileType::CharDevice
    } else if ft.is_fifo() {
        FileType::NamedPipe
    } else if ft.is_socket() {
        FileType::Socket
    } else {
        FileType::RegularFile
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
