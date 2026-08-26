//! Bare-metal MCU layer for Alumy.
//!
//! This module is for firmware without an RTOS. It stays no_std-compatible
//! and re-exports the shared core helpers.

#[doc(no_inline)]
pub use crate::{build_path, crate_name, crate_version, fs, version};
