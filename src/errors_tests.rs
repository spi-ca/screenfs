//! Regression tests for shared error helper behavior.

use super::*;

// Test cases are grouped by the behavior named in each function.
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
