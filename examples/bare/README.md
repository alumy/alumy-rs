# Bare-Metal MCU

This no_std crate contains the portable part of a firmware super loop. Call
`run_once` from the board's reset/runtime entry point.

Check it from the workspace root:

```bash
cargo check -p alumy-example-bare
cargo check -p alumy-example-bare --target thumbv7em-none-eabihf
```
