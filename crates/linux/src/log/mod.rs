//! High-performance, non-blocking logging built on [`tracing`].
//!
//! The main entry point is [`LogConfig`], which provides a fluent builder API
//! for configuring log level, output destination, rotation policy, and format.
//!
//! # Quick start
//!
//! ```no_run
//! use alumy_linux::LogConfig;
//!
//! LogConfig::new("my-app", "info")
//!     .with_file("logs/app.log", "10M", 5)
//!     .init()
//!     .unwrap();
//! ```

#[doc(hidden)]
mod log_init;

#[doc(inline)]
pub use log_init::LogConfig;
