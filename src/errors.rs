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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardDecision {
    Allowed,
    Hidden,
    ReadOnly,
}

impl GuardDecision {
    pub fn errno(self) -> Option<i32> {
        match self {
            GuardDecision::Allowed => None,
            GuardDecision::Hidden => Some(libc::ENOENT),
            GuardDecision::ReadOnly => Some(libc::EROFS),
        }
    }
}

pub fn classify_hidden_readonly(hidden: bool, readonly: bool, mutation: bool) -> GuardDecision {
    if hidden {
        GuardDecision::Hidden
    } else if readonly && mutation {
        GuardDecision::ReadOnly
    } else {
        GuardDecision::Allowed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_write_intent_open_flags() {
        assert!(!open_has_write_intent(libc::O_RDONLY as u32));
        assert!(!open_has_write_intent(
            (libc::O_RDONLY | libc::O_EXCL) as u32
        ));
        assert!(open_has_write_intent(libc::O_WRONLY as u32));
        assert!(open_has_write_intent(libc::O_RDWR as u32));
        assert!(open_has_write_intent(
            (libc::O_RDONLY | libc::O_TRUNC) as u32
        ));
        assert!(open_has_write_intent(
            (libc::O_RDONLY | libc::O_CREAT) as u32
        ));
    }

    #[test]
    fn hidden_precedes_readonly_for_reads_and_mutations() {
        assert_eq!(
            classify_hidden_readonly(true, true, false),
            GuardDecision::Hidden
        );
        assert_eq!(
            classify_hidden_readonly(true, true, true),
            GuardDecision::Hidden
        );
        assert_eq!(
            classify_hidden_readonly(false, true, true),
            GuardDecision::ReadOnly
        );
        assert_eq!(
            classify_hidden_readonly(false, true, false),
            GuardDecision::Allowed
        );
    }

    #[test]
    fn guard_decisions_map_to_expected_errno() {
        assert_eq!(GuardDecision::Allowed.errno(), None);
        assert_eq!(GuardDecision::Hidden.errno(), Some(libc::ENOENT));
        assert_eq!(GuardDecision::ReadOnly.errno(), Some(libc::EROFS));
    }
}
