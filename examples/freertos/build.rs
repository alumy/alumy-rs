//! Compiles and links a vendored FreeRTOS kernel into this firmware binary,
//! and makes `memory.x` visible to the linker.
//!
//! This requires:
//! 1. A vendored copy of the FreeRTOS kernel source (see README.md for how
//!    to fetch it) at the path passed to `Builder::freertos` below.
//! 2. An ARM cross-compiler (e.g. `arm-none-eabi-gcc`) available on PATH.
//! 3. `src/FreeRTOSConfig.h` filled in for your real target chip.
//!
//! cortex-m-rt's own build script emits a `link.x` that does
//! `INCLUDE memory.x`; the memory.x section below copies our `memory.x`
//! into `OUT_DIR` and adds that directory to the linker search path so
//! the include resolves.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut builder = freertos_cargo_build::Builder::new();

    // Path to the vendored FreeRTOS-Kernel source tree (see README.md).
    builder.freertos("FreeRTOS-Kernel");
    // Directory containing FreeRTOSConfig.h.
    builder.freertos_config("src");
    // Match this to your target core, e.g. GCC/ARM_CM4F for Cortex-M4F.
    builder.freertos_port("GCC/ARM_CM4F");

    // The vendored FreeRTOS-Kernel port.c has a benign unused-variable
    // warning under some configs (pxVectorTable in xPortStartScheduler).
    // It's third-party source we don't own, so silence it at the
    // compiler-flag level rather than patching the vendored file.
    builder.get_cc().flag_if_supported("-Wno-unused-variable");

    builder
        .compile()
        .unwrap_or_else(|e| panic!("failed to build FreeRTOS kernel: {e}"));

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}