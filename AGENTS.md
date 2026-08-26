# Alumy Agent Guide

## Workspace

- Keep root `src/lib.rs` as the public facade.
- Put shared no_std-compatible code in `crates/core`.
- Put platform code in `crates/linux`, `crates/bare`, `crates/freertos`, or
  `crates/embassy`; re-export shared helpers instead of duplicating them.
- `linux` is the default root feature. MCU consumers disable default features
  and select `bare`, `freertos`, or `embassy`.
- Keep platform examples as workspace members under `examples`; MCU examples
  must remain no_std-compatible and independently cross-checkable.

## Coding Rules

- Preserve Rust 1.70 compatibility unless the manifest is intentionally updated.
- Keep public API changes conservative.
- Add or update rustdoc for public items.
- Keep `crates/core`, `crates/bare`, `crates/freertos`, and `crates/embassy` no_std-compatible by default.
- Prefer small, dependency-light implementations.

## Validation

Run before finishing:

```bash
cargo fmt --all --check
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

For platform or feature changes, also run:

```bash
cargo check -p alumy --features linux
cargo check -p alumy --no-default-features --features bare
cargo check -p alumy --no-default-features --features freertos
cargo check -p alumy --no-default-features --features embassy
```

If a command cannot be run, report the reason and residual risk.
