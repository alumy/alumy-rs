---
name: alumy-rs
description: Maintain and evolve the alumy Rust workspace. Use when Codex works in the alumy-rs repository on cross-platform Linux/MCU layers, logging, filesystem utilities, system uptime, version metadata, public API docs, tests, cargo checks, or crate release hygiene.
---

# Alumy Rust Workspace

## Overview

Use this skill to make changes in the `alumy` workspace while preserving its compact public API, platform boundaries, documentation quality, and test coverage.

## Repository Shape

- Keep root `src/lib.rs` as the public facade crate surface. It enables `#![warn(missing_docs)]`, re-exports `alumy-core` helpers on every target, and re-exports platform crates behind features.
- Keep workspace members concise and platform-oriented:
  - `crates/core`: shared no_std-friendly helpers.
  - `crates/linux`: Linux/desktop integrations that need `std`, including logging and system uptime.
  - `crates/bare`: bare-metal MCU layer, no_std by default.
  - `crates/freertos`: FreeRTOS MCU layer, no_std by default.
  - `crates/embassy`: Embassy MCU layer, no_std by default.
- Keep root crate features aligned with the platform crate names: `linux` is default; `bare`, `freertos`, and `embassy` are MCU options used with `default-features = false`.
- Put OS-independent filesystem helpers under `crates/core/src/fs/`; update `crates/core/src/lib.rs` when adding modules.
- Put Linux logging changes under `crates/linux/src/log/`; `LogConfig` lives in `crates/linux/src/log/log_init.rs` and is re-exported through `crates/linux/src/log/mod.rs`, `crates/linux/src/lib.rs`, and root `src/lib.rs` behind the `linux` feature.
- Put Linux system helpers under `crates/linux/src/sys/`; keep platform-specific code behind `cfg` blocks.
- Keep version metadata helpers in `crates/core/src/version.rs`.
- Put MCU platform adapters in their matching platform crate instead of the root facade.
- Add integration tests under root `tests/` for public facade behavior. Use unit tests inside the owning crate for private helpers or narrow module-local edge cases.

## Coding Guidelines

- Preserve Rust 1.70 compatibility unless the workspace manifest is intentionally updated.
- Add or update rustdoc for every public item; missing docs are warnings for these crates.
- Prefer small, dependency-light implementations. Do not add dependencies for simple parsing, formatting, or OS calls without a clear reason.
- Keep `crates/core`, `crates/bare`, `crates/freertos`, and `crates/embassy` no_std-compatible by default.
- Return `anyhow::Result` where the existing Linux logging API already does so, and use `Option` for parser-style helpers that currently expose `Option`.
- Keep public API changes conservative. When adding a new public helper, update README examples if users should discover it.

## Module Notes

### Platform Facade

- Root `alumy` defaults to `linux` for backwards compatibility with existing Linux users.
- MCU users should use `default-features = false` with exactly one or more MCU features as needed: `bare`, `freertos`, or `embassy`.
- Avoid duplicating shared helpers in platform crates; re-export from `alumy-core` unless the platform needs different behavior.

### Logging

- Linux logging lives in `crates/linux/src/log/`.
- `LogConfig::new(name, level)` sets required fields; `init()` delegates to `logger_init`.
- File logging requires `file`, `max_size`, and `max_files`; `with_file(path, max_size, max_files)` is the normal path.
- `with_filter()` overrides `level` when the filter string is non-empty. Invalid filters fall back to `info`.
- `time_format == "uptime"` uses `sys::uptime::uptime_duration`; other enabled timestamps use local ISO-like time.
- `LOG_GUARD` keeps the non-blocking writer alive globally. Be careful when tests initialize the global tracing subscriber, because only one global subscriber can be installed per process.
- Rolling file output derives the final file name from the file stem and writes `<stem>.log` in the parent directory.

### Filesystem

- Core filesystem helpers live in `crates/core/src/fs/`.
- `fs::filesize::parse_size` accepts integer-only base-1024 sizes and recognizes `B`, `K`, `KB`, `KiB`, through `P`, `PB`, `PiB`, case-insensitively.
- `fs::filesize::format_size` emits compact base-1024 strings such as `512B`, `1.0KB`, and `10.0MB`; it is available when `alumy-core` has the `alloc` feature.
- Keep path-building helpers platform-neutral where possible.

### System

- Linux system helpers live in `crates/linux/src/sys/`.
- `sys::uptime` uses `clock_gettime(CLOCK_MONOTONIC)` on Unix, `GetTickCount64` on Windows, and `Duration::ZERO` elsewhere.
- Avoid tests that assume exact uptime values. Assert monotonic or positive properties only.

## Validation

Run the narrowest useful check first, then broaden before finishing:

```bash
cargo fmt --all --check
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

For platform feature changes, also run relevant checks:

```bash
cargo check -p alumy --features linux
cargo check -p alumy --no-default-features --features bare
cargo check -p alumy --no-default-features --features freertos
cargo check -p alumy --no-default-features --features embassy
```

For release-facing changes, also run:

```bash
cargo package --allow-dirty
```

If a command cannot be run, report the reason and the residual risk.
