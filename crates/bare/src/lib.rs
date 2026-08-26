#![no_std]
#![warn(missing_docs)]
//! Bare-metal MCU layer for Alumy.
//!
//! This crate is for firmware without an RTOS. It stays no_std by default and
//! re-exports the shared core helpers.

#[doc(no_inline)]
pub use alumy_core::{build_path, crate_name, crate_version, fs, version};
