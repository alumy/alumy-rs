# alumy

[![Crates.io](https://img.shields.io/crates/v/alumy.svg)](https://crates.io/crates/alumy)
[![Documentation](https://docs.rs/alumy/badge.svg)](https://docs.rs/alumy)
[![License](https://img.shields.io/crates/l/alumy.svg)](https://github.com/alumy/alumy-rs/blob/main/LICENSE)

A compact cross-platform Rust SDK for Linux and MCU development.

## Workspace

```text
alumy/
├── Cargo.toml
├── src/
│   ├── core
│   └── linux
├── examples/
│   ├── linux
│   ├── bare
│   ├── freertos
│   └── embassy
└── tests
```

## Platforms

- `fs` and `version`: no_std-friendly shared helpers.
- `linux`: default Linux/desktop layer with logging, filesystem helpers, and uptime.
- `bare`: bare-metal MCU layer, no_std by default.
- `freertos`: FreeRTOS MCU layer, no_std by default.
- `embassy`: Embassy MCU layer, no_std by default.

Only the root `alumy` crate is published. Applications select one platform
layer with Cargo features instead of depending on platform-specific crates.

## Features

- `linux`: enabled by default.
- `bare`: enables the bare-metal MCU layer.
- `freertos`: enables the FreeRTOS MCU layer.
- `embassy`: enables the Embassy MCU layer.

## Installation

Linux/default:

```toml
[dependencies]
alumy = "0.1.14"
anyhow = "1"
```

Bare-metal MCU:

```toml
[dependencies]
alumy = { version = "0.1.14", default-features = false, features = ["bare"] }
```

FreeRTOS MCU:

```toml
[dependencies]
alumy = { version = "0.1.14", default-features = false, features = ["freertos"] }
```

Embassy MCU:

```toml
[dependencies]
alumy = { version = "0.1.14", default-features = false, features = ["embassy"] }
```

## Usage

### Linux Logging

```rust
use alumy::{debug, info, LogConfig};

fn main() -> anyhow::Result<()> {
    LogConfig::new("my-app", "debug")
        .with_file("logs/app.log", "10M", 5)
        .with_time_format("uptime")
        .with_ansi(true)
        .with_target(true)
        .init()?;

    info!("Hello, alumy logger!");
    debug!("Debug message");
    Ok(())
}
```

`LogConfig::init` installs a process-wide tracing subscriber and should be
called once during application startup.

### System Uptime

```rust
use alumy::sys::uptime;

fn main() {
    println!("Uptime: {} seconds", uptime::uptime());
    println!("Uptime duration: {:?}", uptime::uptime_duration());
}
```

### Core Helpers

```rust
use alumy::fs::filesize;
use alumy::{crate_name, crate_version};

fn main() {
    let size = filesize::parse_size("10M").unwrap();
    println!("{} {}: {} bytes", crate_name!(), crate_version!(), size);
}
```

## Examples

The `examples` workspace members are compiled by the normal workspace checks:

```bash
cargo run -p alumy-example-linux
cargo check -p alumy-example-bare
cargo check -p alumy-example-freertos
cargo check -p alumy-example-embassy
```

The MCU examples are no_std libraries containing portable application logic.
Board startup, HAL selection, interrupt vectors, and executor or RTOS bindings
remain in the target firmware crate.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Please feel free to submit a Pull Request.

## Links

- [GitHub Repository](https://github.com/alumy/alumy-rs)
- [Documentation](https://docs.rs/alumy)
- [Crates.io](https://crates.io/crates/alumy)
