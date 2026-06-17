//! Error helpers shared by FUSE operation handlers.
//!
//! Keep host errno preservation and write-intent detection small and explicit so
//! policy code can decide when to return `ENOENT` or `EROFS`.

use std::io;

use fractal_fuse::EIO;

// Preserve host errno values whenever policy did not choose a synthetic error.
pub fn errno_from_io(err: io::Error) -> i32 {
    err.raw_os_error().unwrap_or(EIO)
}

// Treat create/truncate flags as write intent even when the access mode is read-only.
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
