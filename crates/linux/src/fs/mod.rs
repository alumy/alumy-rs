//! Filesystem utilities for path building and file-size handling.
//!
//! `alumy-linux` re-exports the OS-independent filesystem helpers from
//! `alumy-core` so Linux users keep the same module paths as the top-level
//! `alumy` facade.

pub use alumy_core::fs::{filesize, path};
