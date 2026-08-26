#![cfg_attr(not(feature = "linux"), no_std)]
#![warn(missing_docs)]
//! # alumy
//!
//! A compact cross-platform Rust SDK facade for Linux and MCU development.
//!
//! `alumy` always exports no_std-friendly helpers. Platform integrations are
//! selected through concise features: `linux`, `bare`, `freertos`, and
//! `embassy`.

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use crate::core::fs;
#[doc(inline)]
pub use crate::core::version;

#[cfg(feature = "bare")]
pub mod bare;

#[cfg(feature = "embassy")]
pub mod embassy;

#[cfg(feature = "freertos")]
pub mod freertos;

#[cfg(feature = "linux")]
pub mod linux;

mod core;

#[cfg(feature = "linux")]
#[doc(inline)]
pub use linux::{log, sys, LogConfig};

#[cfg(feature = "linux")]
#[doc(no_inline)]
pub use linux::{debug, error, info, trace, warn, Level};
