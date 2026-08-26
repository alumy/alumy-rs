#![no_std]
#![no_main]
//! Flashable bare-metal firmware entry point built on Alumy's no_std APIs.
//!
//! Board bring-up (clocks, GPIO, peripherals) is intentionally left out:
//! add it here once you target a specific chip. This crate only wires up
//! the cortex-m-rt entry point, a panic handler, and the portable Alumy
//! helpers so it links into a real ELF for the configured target.

use alumy::bare::{build_path, fs};
use cortex_m_rt::entry;
use panic_halt as _;

/// Conventional firmware configuration path.
const CONFIG_PATH: &str = build_path!("/etc", "firmware", ".conf");

#[entry]
fn main() -> ! {
    let _dma_buffer_size = fs::filesize::parse_size("4KiB");
    let _config_path = CONFIG_PATH;

    loop {
        // Board-specific super loop body goes here.
        cortex_m::asm::wfi();
    }
}
