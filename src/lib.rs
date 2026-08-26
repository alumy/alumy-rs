#![cfg_attr(not(feature = "linux"), no_std)]
#![warn(missing_docs)]
//! # alumy
//!
//! A compact cross-platform Rust SDK facade for Linux and MCU development.
//!
//! `alumy` always exports no_std-friendly helpers from `alumy-core`. Platform
//! integrations are selected through concise features: `linux`, `bare`,
//! `freertos`, and `embassy`.

#[doc(inline)]
pub use alumy_core::fs;
#[doc(inline)]
pub use alumy_core::version;
#[doc(no_inline)]
pub use alumy_core::{build_path, crate_name, crate_version};

#[cfg(feature = "bare")]
#[doc(inline)]
pub use alumy_bare as bare;

#[cfg(feature = "embassy")]
#[doc(inline)]
pub use alumy_embassy as embassy;

#[cfg(feature = "freertos")]
#[doc(inline)]
pub use alumy_freertos as freertos;

#[cfg(feature = "linux")]
#[doc(inline)]
pub use alumy_linux::{log, sys, LogConfig};

#[cfg(feature = "linux")]
#[doc(no_inline)]
pub use alumy_linux::{debug, error, info, trace, warn, Level};
