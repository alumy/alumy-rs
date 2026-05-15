#![warn(missing_docs)]
//! # alumy
//!
//! A batteries-included Rust SDK for rapid application development.
//! Logging, system utilities, and filesystem helpers — all ready to use.
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
//! [`LogConfig`] is re-exported at the crate root for convenience.
//! The following items from [`tracing`] are also re-exported:
//! [`trace!`], [`debug!`], [`info!`], [`warn!`], [`error!`], and [`Level`].

pub mod version;
pub mod fs;
pub mod sys;
pub mod log;

#[doc(inline)]
pub use log::LogConfig;

#[doc(no_inline)]
pub use tracing::{trace, debug, info, warn, error, Level};
