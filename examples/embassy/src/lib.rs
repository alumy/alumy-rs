#![no_std]
//! Minimal Embassy task integration using no_std-compatible Alumy APIs.

use alumy::embassy::fs;

/// Portable async task body for an Embassy executor task.
pub async fn task() -> Option<u64> {
    fs::filesize::parse_size("16KiB")
}

/// Synchronous setup that can run before starting the Embassy executor.
pub fn setup() -> Option<u64> {
    fs::filesize::parse_size("16KiB")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_embassy_state() {
        assert_eq!(setup(), Some(16 * 1024));
    }
}
