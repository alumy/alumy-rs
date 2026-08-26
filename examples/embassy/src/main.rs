#![no_std]
#![no_main]
//! Flashable Embassy firmware entry point built on Alumy's no_std APIs.
//!
//! Targets a generic STM32F411 (Cortex-M4F). `embassy_stm32::init` brings
//! up the chip's clocks and peripherals, which also supplies the
//! `embassy-time` time driver and the Cortex-M critical-section
//! implementation this binary links against.

use alumy::embassy::fs;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _peripherals = embassy_stm32::init(Default::default());
    let _buffer_size = fs::filesize::parse_size("16KiB");

    loop {
        // Board-specific async task body goes here.
        Timer::after_secs(1).await;
    }
}