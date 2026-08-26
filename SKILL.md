---
name: alumy-rs
description: Maintain the Alumy Rust workspace across Linux, bare-metal, FreeRTOS, and Embassy targets. Use for code, tests, documentation, validation, releases, and repository guidance.
---

# Alumy Rust Workspace

Read and follow `AGENTS.md` before making changes. Treat it as the shared source
of truth for architecture, compatibility, and validation.

## Architecture

- This is one publishable crate: `alumy`. Platform selection is feature-based.
- `linux` is the default feature; MCU users disable default features and select
  `bare`, `freertos`, or `embassy`.
- Shared no_std code lives in `src/core` and is exposed as `alumy::{fs, version}`.
- Platform namespaces live in `src/linux`, `src/bare`, `src/freertos`, and
  `src/embassy`.
- `tests/platform.rs` is the single auto-discovered integration-test entry
  point. Shared tests belong in `tests/core`; platform-specific tests belong in
  their platform directory. Linux logger cases use child-process isolation.
- `examples/linux` is a host executable. The MCU examples are no_std firmware
  binaries and target `thumbv7em-none-eabihf` for the checked-in skeleton.

## Validation

Run the complete matrix before finishing platform work:

```bash
cargo fmt --all --check
cargo test --workspace --all-features
cargo test -p alumy --no-default-features --features bare
cargo test -p alumy --no-default-features --features freertos
cargo test -p alumy --no-default-features --features embassy
cargo check -p alumy --features linux
cargo check -p alumy --no-default-features --features bare
cargo check -p alumy --no-default-features --features freertos
cargo check -p alumy --no-default-features --features embassy
cargo build -p alumy-example-linux
cargo build -p alumy-example-bare --target thumbv7em-none-eabihf
cargo build -p alumy-example-freertos --target thumbv7em-none-eabihf
cargo build -p alumy-example-embassy --target thumbv7em-none-eabihf
cargo doc --workspace --all-features --no-deps
```

Host `cargo test` builds the MCU examples through their host fallback; the
targeted ARM builds verify the actual firmware entry points and linker setup.
Use the generated ELF with the board's flashing/debug probe after updating its
`memory.x` and chip-specific HAL configuration.

Before publishing, run `cargo publish --dry-run -p alumy --registry crates-io`
and confirm that only the root `alumy` package is being prepared.
