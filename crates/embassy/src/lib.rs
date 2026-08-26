#![no_std]
#![warn(missing_docs)]
//! Embassy MCU layer for Alumy.
//!
//! This crate is the home for Embassy-specific async adapters. It stays no_std
//! by default and re-exports the shared core helpers.

#[doc(no_inline)]
pub use alumy_core::{build_path, crate_name, crate_version, fs, version};
