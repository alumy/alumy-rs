#![no_std]
#![no_main]
//! Flashable Embassy firmware entry point built on Alumy's no_std APIs.
//!
//! Board bring-up and peripheral HAL selection are intentionally left out:
//! add an embassy-<chip-family> HAL and its `embassy_time` driver here once
//! you target a specific chip. This crate only wires up the executor entry
//! point, a panic handler, and the portable Alumy helpers so it links into
//! a real ELF for the configured target.

use alumy::embassy::fs;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _buffer_size = fs::filesize::parse_size("16KiB");

    loop {
        // Board-specific async task body goes here.
        Timer::after_secs(1).await;
    }
}
