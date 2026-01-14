# alumy

[![Crates.io](https://img.shields.io/crates/v/alumy.svg)](https://crates.io/crates/alumy)
[![Documentation](https://docs.rs/alumy/badge.svg)](https://docs.rs/alumy)
[![License](https://img.shields.io/crates/l/alumy.svg)](https://github.com/alumy/alumy-rs/blob/main/LICENSE)

Rust SDK for embedded systems development, providing safe and efficient hardware abstraction and device control.

## Features

- **Version Management**: Macros and functions to access crate metadata at compile time.
- **Filesystem Utilities**: Path building and filesystem helpers (under development).
- **Embedded Focused**: Designed for `no_std` environments and efficient hardware control.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
alumy = "0.1"
```

## Usage

### Version Information

You can access the crate version and name using functions or macros:

```rust
use alumy::version;

fn main() {
    println!("Crate: {}", version::name());
    println!("Version: {}", version::version());
    println!("{}", version::hello());
}
```

### Macros

The crate provides macros for compile-time constants:

```rust
use alumy::{crate_name, crate_version};

fn main() {
    println!("Running {} v{}", crate_name!(), crate_version!());
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
