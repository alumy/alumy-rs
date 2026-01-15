//! # alumy
//!
//! A cross-platform easy-start SDK for Rust development. Provides essential libraries to accelerate application development.
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
