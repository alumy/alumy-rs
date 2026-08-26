//! Compiles and links a vendored FreeRTOS kernel into this firmware binary.
//!
//! This requires:
//! 1. A vendored copy of the FreeRTOS kernel source (see README.md for how
//!    to fetch it) at the path passed to `Builder::freertos` below.
//! 2. An ARM cross-compiler (e.g. `arm-none-eabi-gcc`) available on PATH.
//! 3. `src/FreeRTOSConfig.h` filled in for your real target chip.

fn main() {
    let mut builder = freertos_cargo_build::Builder::new();

    // Path to the vendored FreeRTOS-Kernel source tree (see README.md).
    builder.freertos("FreeRTOS-Kernel");
    // Directory containing FreeRTOSConfig.h.
    builder.freertos_config("src");
    // Match this to your target core, e.g. GCC/ARM_CM4F for Cortex-M4F.
    builder.freertos_port("GCC/ARM_CM4F");

    builder
        .compile()
        .unwrap_or_else(|e| panic!("failed to build FreeRTOS kernel: {e}"));
}
