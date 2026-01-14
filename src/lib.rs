//! # alumy
//!
//! Rust SDK for embedded systems development, providing safe and efficient hardware abstraction and device control.

pub mod version;
pub mod fs;
pub mod sys;
pub mod log;

pub use log::{LogConfig, logger_init};
