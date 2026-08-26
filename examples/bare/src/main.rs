#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]
//! Flashable bare-metal firmware entry point built on Alumy's no_std APIs.
//!
//! Board bring-up (clocks, GPIO, peripherals) is intentionally left out:
//! add it here once you target a specific chip. This crate only wires up
//! the cortex-m-rt entry point, a panic handler, and the portable Alumy
//! helpers so it links into a real ELF for the configured target.

#[cfg(target_arch = "arm")]
use alumy::bare::{build_path, fs};
#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;
#[cfg(target_arch = "arm")]
use panic_halt as _;

/// Conventional firmware configuration path.
#[cfg(target_arch = "arm")]
const CONFIG_PATH: &str = build_path!("/etc", "firmware", ".conf");

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    let _dma_buffer_size = fs::filesize::parse_size("4KiB");
    let _config_path = CONFIG_PATH;

    loop {
        // Board-specific super loop body goes here.
        cortex_m::asm::wfi();
    }
}

#[cfg(not(target_arch = "arm"))]
fn main() {}
