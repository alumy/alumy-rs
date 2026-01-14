//! # alumy
//!
//! Rust SDK for embedded systems development, providing safe and efficient hardware abstraction and device control.
//!
//! ## Modules
//!
//! - [`log`]: High-performance, non-blocking logging utilities with fluent configuration.
//! - [`sys`]: System-level utilities like uptime.
//! - [`fs`]: Filesystem utilities including size parsing and path building.
//! - [`version`]: Crate metadata and version information.

pub mod version;
pub mod fs;
pub mod sys;
pub mod log;

#[doc(inline)]
pub use log::LogConfig;
