pub mod cli;
pub mod config;
pub mod errors;
pub mod fs;
pub mod matcher;
pub mod path;

pub use cli::{LaunchArgs, MutabilityDefault, VisibilityDefault};
pub use config::{PolicySource, RuntimeConfig, VisibilityDecision};
pub use fs::ScreenFs;

pub fn ensure_non_root_user() -> std::io::Result<()> {
    if is_root_euid(current_euid()) {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "screenfs must be run as a non-root user",
        ))
    } else {
        Ok(())
    }
}

fn current_euid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and cannot fail.
    unsafe { libc::geteuid() }
}

fn is_root_euid(euid: u32) -> bool {
    euid == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_euid_detection_is_exact() {
        assert!(is_root_euid(0));
        assert!(!is_root_euid(1));
        assert!(!is_root_euid(1000));
    }
}
