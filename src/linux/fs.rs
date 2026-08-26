//! Filesystem utilities for path building and file-size handling.
//!
//! Linux re-exports the OS-independent filesystem helpers so users keep the
//! same module paths as the top-level `alumy` facade.

pub use crate::fs::{filesize, path};
