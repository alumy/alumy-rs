#![no_std]
#![no_main]
//! Flashable FreeRTOS firmware entry point built on Alumy's no_std APIs.
//!
//! Board bring-up and the FreeRTOS port layer are intentionally left out:
//! see build.rs and README.md for what's needed to link a real FreeRTOS
//! kernel into this binary. This crate only wires up the cortex-m-rt entry
//! point, a panic handler, a single FreeRTOS task, and the portable Alumy
//! helpers.

use alumy::freertos::fs;
use cortex_m_rt::entry;
use freertos_rust::{Duration, FreeRtosUtils, Task, CurrentTask};
use panic_halt as _;

#[entry]
fn main() -> ! {
    Task::new()
        .name("alumy_task")
        .stack_size(256)
        .start(|| {
            let _stack_bytes = fs::filesize::parse_size("8KiB");
            loop {
                // Board-specific task body goes here.
                CurrentTask::delay(Duration::ms(1000));
            }
        })
        .expect("failed to create alumy_task");

    FreeRtosUtils::start_scheduler();
}
