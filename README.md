# alumy

[![Crates.io](https://img.shields.io/crates/v/alumy.svg)](https://crates.io/crates/alumy)
[![Documentation](https://docs.rs/alumy/badge.svg)](https://docs.rs/alumy)
[![License](https://img.shields.io/crates/l/alumy.svg)](https://github.com/alumy/alumy-rs/blob/main/LICENSE)

An easy-start SDK for Rust development. Provides essential libraries to accelerate application development.

## Features

- **High-Performance Logging**: Non-blocking logger based on `tracing` with fluent configuration API, log rotation, and system uptime timestamps. Works across platforms.
- **System Utilities**: Helpers for system information such as uptime (supports Linux, macOS, and Windows).
- **Filesystem Utilities**: Size parsing/formatting and path building helpers.
- **Version Management**: Macros and functions to access crate metadata at compile time.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
alumy = "0.1"
```

## Usage

### Logging Setup

Alumy provides a modern, non-blocking logger based on `tracing`. It supports a fluent API for easy configuration:

```rust
use alumy::{LogConfig, info, debug};

fn main() -> anyhow::Result<()> {
    // Basic setup
    LogConfig::new("my-app", "info").init()?;

    // Advanced setup with log rotation and system uptime
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

Access system uptime information:

```rust
use alumy::sys::uptime;

fn main() {
    println!("Uptime: {} seconds", uptime::uptime());
    println!("Uptime duration: {:?}", uptime::uptime_duration());
}
```

### Filesystem Utilities

Parse and format file sizes easily:

```rust
use alumy::fs::filesize;

fn main() {
    let size = filesize::parse_size("10M").unwrap();
    println!("10M in bytes: {}", size);
    println!("Formatted: {}", filesize::format_size(size)); // "10.0MB"
}
```

### Version Information

Access crate metadata:

```rust
use alumy::version;
use alumy::{crate_name, crate_version};

fn main() {
    println!("Running {} v{}", crate_name!(), crate_version!());
    println!("{}", version::hello());
}
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [GitHub Repository](https://github.com/alumy/alumy-rs)
- [Documentation](https://docs.rs/alumy)
- [Crates.io](https://crates.io/crates/alumy)
