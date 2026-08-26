# Embassy MCU

This no_std crate provides a portable async task body. Annotate a thin wrapper
with the target runtime's `#[embassy_executor::task]` attribute and await
`task()` from that wrapper.

Check it from the workspace root:

```bash
cargo check -p alumy-example-embassy
cargo check -p alumy-example-embassy --target thumbv7em-none-eabihf
```
