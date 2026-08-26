# alumy

[![Crates.io](https://img.shields.io/crates/v/alumy.svg)](https://crates.io/crates/alumy)
[![Documentation](https://docs.rs/alumy/badge.svg)](https://docs.rs/alumy)
[![License](https://img.shields.io/crates/l/alumy.svg)](https://github.com/alumy/alumy-rs/blob/main/LICENSE)

`alumy` is a small cross-platform Rust library for Linux and MCU firmware.
The project publishes one crate and selects platform integrations with Cargo
features.

## Platforms

| Feature | Target | Role |
| --- | --- | --- |
| `linux` | Linux and desktop | Logging, filesystem helpers, and uptime |
| `bare` | Bare-metal MCU | no_std platform extension point |
| `freertos` | MCU with FreeRTOS | no_std RTOS extension point |
| `embassy` | MCU with Embassy | no_std async extension point |

`linux` is enabled by default. MCU applications should disable default
features and select one MCU feature.

## Install

Linux or desktop:

```toml
[dependencies]
alumy = "0.1.15"
anyhow = "1"
```

MCU application:

```toml
[dependencies]
alumy = { version = "0.1.15", default-features = false, features = ["bare"] }
```

Replace `bare` with `freertos` or `embassy` as needed. Only the root `alumy`
crate is published; platform layers are not separate crates.

## API

The shared API is available on every platform:

```rust
use alumy::fs::filesize;
use alumy::{crate_name, crate_version};

fn main() {
    let size = filesize::parse_size("10M").unwrap();
    println!("{} {}: {} bytes", crate_name!(), crate_version!(), size);
}
```

Linux logging and uptime are enabled with the default feature:

```rust
use alumy::{info, LogConfig};

fn main() -> anyhow::Result<()> {
    LogConfig::new("my-app", "info")
        .with_file("logs/app.log", "10M", 5)
        .with_time_format("uptime")
        .init()?;

    info!("Hello, alumy!");
    Ok(())
}
```

`LogConfig::init` installs a process-wide tracing subscriber and should be
called once during application startup.

## Repository Layout

```text
src/
├── core/       Shared no_std implementation
├── linux/      Linux implementation
├── bare/       Bare-metal namespace
├── freertos/   FreeRTOS namespace
└── embassy/    Embassy namespace

examples/
├── linux/      Host executable
├── bare/       Cortex-M firmware binary
├── freertos/   FreeRTOS firmware binary
└── embassy/    Embassy firmware binary

tests/
├── core/       Shared integration tests
├── linux/      Linux-specific tests
├── bare/       Bare-metal test extension point
├── freertos/   FreeRTOS test extension point
├── embassy/    Embassy test extension point
└── platform.rs Single feature-gated test entry point
```

## Build And Test

Run the complete release validation from the repository root:

```bash
./release_check.sh
```

The script checks all library features, tests, host examples, ARM firmware
examples, documentation, and the `alumy` crates.io publish dry-run.

The MCU examples use `thumbv7em-none-eabihf` and produce ELF firmware files.
Update each example's `memory.x` and board-specific HAL configuration before
flashing with a debug probe.

For a quick local test run:

```bash
cargo test --workspace --all-features
```

## License

Licensed under the [MIT License](LICENSE).

## Links

- [Repository](https://github.com/alumy/alumy-rs)
- [Documentation](https://docs.rs/alumy)
- [Crates.io](https://crates.io/crates/alumy)
