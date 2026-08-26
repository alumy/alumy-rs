---
name: alumy-rs
description: Maintain the Alumy Rust workspace across Linux, bare-metal, FreeRTOS, and Embassy targets. Use for code, tests, documentation, validation, releases, and repository guidance.
---

# Alumy Rust Workspace

This repository publishes one crate, `alumy`; platforms are selected with
`linux` (default), `bare`, `freertos`, or `embassy`.

Shared no_std code belongs in `src/core`; platform code belongs in `src/linux`,
`src/bare`, `src/freertos`, or `src/embassy`. Shared integration tests belong
in `tests/core`, platform tests in their directory, and `tests/platform.rs` is
the single feature-gated test entry point.

Run `./release_check.sh` before release. It is the source of truth for library
features, all tests, host and ARM examples, firmware linking, docs, and the
root-crate publish dry-run. ARM firmware uses
`thumbv7em-none-eabihf`; update `memory.x` for the target board before flashing.
