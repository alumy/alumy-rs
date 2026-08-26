# FreeRTOS MCU

This no_std crate shows the portable body and configuration for an application
task. Connect `task_entry` to the FreeRTOS binding used by the target firmware.

Check it from the workspace root:

```bash
cargo check -p alumy-example-freertos
cargo check -p alumy-example-freertos --target thumbv7em-none-eabihf
```
