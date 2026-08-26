//! Makes `memory.x` visible to the linker.
//!
//! cortex-m-rt's own build script emits a `link.x` that does
//! `INCLUDE memory.x`; this copies our `memory.x` into `OUT_DIR` and adds
//! that directory to the linker search path so the include resolves.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
