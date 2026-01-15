//! # alumy
//!
//! A batteries-included Rust SDK for rapid application development. Logging, system utilities, and filesystem helpers — all ready to use.
//!
//! ## Modules
//!
//! - [`log`]: High-performance, non-blocking logging utilities with fluent configuration.
//! - [`sys`]: System-level utilities like uptime.
//! - [`fs`]: Filesystem utilities including size parsing and path building.
//! - [`version`]: Crate metadata and version information.
//!
//! ## Re-exports
//!
//! This crate re-exports logging macros from `tracing` for convenience:
//! `trace!`, `debug!`, `info!`, `warn!`, `error!`, and the `Level` enum.

pub mod version;
pub mod fs;
pub mod sys;
pub mod log;

#[doc(inline)]
pub use log::LogConfig;

pub use tracing::{trace, debug, info, warn, error, Level};
