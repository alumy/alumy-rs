#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]
//! Flashable FreeRTOS firmware entry point built on Alumy's no_std APIs.
//!
//! Board bring-up and the FreeRTOS port layer are intentionally left out:
//! see build.rs and README.md for what's needed to link a real FreeRTOS
//! kernel into this binary. This crate only wires up the cortex-m-rt entry
//! point, a panic handler, the FreeRTOS heap as the global allocator, a
//! single FreeRTOS task, and the portable Alumy helpers.

#[cfg(target_arch = "arm")]
use alumy::freertos::fs;
#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;
#[cfg(target_arch = "arm")]
use freertos_rust::{
    CurrentTask, Duration, FreeRtosAllocator, FreeRtosCharPtr, FreeRtosTaskHandle, FreeRtosUtils,
    Task,
};
#[cfg(target_arch = "arm")]
use panic_halt as _;

/// FreeRTOS's own heap (configured via `configTOTAL_HEAP_SIZE` in
/// `FreeRTOSConfig.h`) backs Rust's global allocator.
#[cfg(target_arch = "arm")]
#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

/// Required because `FreeRTOSConfig.h` sets `configUSE_MALLOC_FAILED_HOOK`.
/// Called by the kernel whenever `pvPortMalloc` returns NULL.
#[cfg(target_arch = "arm")]
#[no_mangle]
pub extern "C" fn vApplicationMallocFailedHook() {
    panic!("FreeRTOS malloc failed");
}

/// Required because `FreeRTOSConfig.h` sets `configCHECK_FOR_STACK_OVERFLOW`.
/// Called by the kernel when a task's stack has overflowed.
#[cfg(target_arch = "arm")]
#[no_mangle]
pub extern "C" fn vApplicationStackOverflowHook(
    _task: FreeRtosTaskHandle,
    _task_name: FreeRtosCharPtr,
) {
    panic!("FreeRTOS stack overflow");
}

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    Task::new()
        .name("alumy_task")
        .stack_size(256)
        .start(|_this_task| {
            let _stack_bytes = fs::filesize::parse_size("8KiB");
            loop {
                // Board-specific task body goes here.
                CurrentTask::delay(Duration::ms(1000));
            }
        })
        .expect("failed to create alumy_task");

    FreeRtosUtils::start_scheduler();
}

#[cfg(not(target_arch = "arm"))]
fn main() {}
