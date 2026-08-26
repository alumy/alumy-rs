# Alumy Agent Guide

## Workspace

- Publish only the root `alumy` crate. Do not add publishable `alumy-*` crates
  for platform layers.
- Keep root `src/lib.rs` as the public facade and feature gate.
- Put shared no_std-compatible code under `src/core` and expose it through the
  root `alumy::{fs, version}` API.
- Put Linux code under `src/linux`; keep MCU platform namespaces in the root
  crate behind `bare`, `freertos`, and `embassy` features.
- `linux` is the default feature. MCU consumers disable default features and
  select `bare`, `freertos`, or `embassy`.
- Keep platform examples as workspace members under `examples`; MCU examples
  must remain no_std-compatible and independently cross-checkable.

## Coding Rules

- Preserve Rust 1.70 compatibility unless the manifest is intentionally updated.
- Keep public API changes conservative.
- Add or update rustdoc for public items.
- Keep shared helpers and MCU feature builds no_std-compatible by default.
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

Before publishing, confirm `cargo publish --dry-run -p alumy --registry crates-io`
does not include or require any `alumy-*` package dependencies.

If a command cannot be run, report the reason and residual risk.
