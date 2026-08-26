# alumy

[![Crates.io](https://img.shields.io/crates/v/alumy.svg)](https://crates.io/crates/alumy)
[![Documentation](https://docs.rs/alumy/badge.svg)](https://docs.rs/alumy)
[![License](https://img.shields.io/crates/l/alumy.svg)](https://github.com/alumy/alumy-rs/blob/main/LICENSE)

A compact cross-platform Rust SDK for Linux and MCU development.

## Workspace

```text
alumy/
├── Cargo.toml
├── crates/
│   ├── core
│   ├── linux
│   ├── bare
│   ├── freertos
│   └── embassy
├── examples/
│   ├── linux
│   ├── bare
│   ├── freertos
│   └── embassy
└── tests
```

## Platforms

- `core`: no_std-friendly shared helpers.
- `linux`: default Linux/desktop layer with logging, filesystem helpers, and uptime.
- `bare`: bare-metal MCU layer, no_std by default.
- `freertos`: FreeRTOS MCU layer, no_std by default.
- `embassy`: Embassy MCU layer, no_std by default.

## Features

- `linux`: enabled by default.
- `bare`: enables the bare-metal MCU layer.
- `freertos`: enables the FreeRTOS MCU layer.
- `embassy`: enables the Embassy MCU layer.

## Installation

Linux/default:

```toml
[dependencies]
alumy = "0.1.13"
anyhow = "1"
```

Bare-metal MCU:

```toml
[dependencies]
alumy = { version = "0.1.13", default-features = false, features = ["bare"] }
```

FreeRTOS MCU:

```toml
[dependencies]
alumy = { version = "0.1.13", default-features = false, features = ["freertos"] }
```

Embassy MCU:

```toml
[dependencies]
alumy = { version = "0.1.13", default-features = false, features = ["embassy"] }
```

## Usage

### Linux Logging

```rust
use alumy::{debug, info, LogConfig};

fn main() -> anyhow::Result<()> {
    LogConfig::new("my-app", "info").init()?;

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

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome. Please feel free to submit a Pull Request.

## Links

- [GitHub Repository](https://github.com/alumy/alumy-rs)
- [Documentation](https://docs.rs/alumy)
- [Crates.io](https://crates.io/crates/alumy)
