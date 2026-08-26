#![no_std]
//! Minimal bare-metal integration using only no_std-compatible Alumy APIs.

use alumy::bare::{build_path, fs};

/// Conventional firmware configuration path.
pub const CONFIG_PATH: &str = build_path!("/etc", "firmware", ".conf");

/// Returns the configured DMA buffer size in bytes.
pub fn dma_buffer_size() -> Option<u64> {
    fs::filesize::parse_size("4KiB")
}

/// Represents one iteration of a bare-metal super loop.
pub fn run_once() -> Option<u64> {
    dma_buffer_size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_bare_metal_state() {
        assert_eq!(CONFIG_PATH, "/etc/firmware/firmware.conf");
        assert_eq!(run_once(), Some(4096));
    }
}
