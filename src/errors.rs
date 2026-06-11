use std::io;

use fractal_fuse::EIO;

pub fn errno_from_io(err: io::Error) -> i32 {
    err.raw_os_error().unwrap_or(EIO)
}

pub fn open_has_write_intent(flags: u32) -> bool {
    let access_mode = flags as i32 & libc::O_ACCMODE;
    access_mode == libc::O_WRONLY
        || access_mode == libc::O_RDWR
        || flags as i32 & libc::O_TRUNC != 0
        || flags as i32 & libc::O_CREAT != 0
}

#[cfg(test)]
#[path = "errors_tests.rs"]
mod tests;
