#![warn(missing_docs)]
//! Linux integrations for Alumy.

pub mod fs;
pub mod log;
pub mod sys;

#[doc(inline)]
pub use log::LogConfig;

#[doc(no_inline)]
pub use alumy_core::{build_path, crate_name, crate_version, version};

#[doc(no_inline)]
pub use tracing::{debug, error, info, trace, warn, Level};
