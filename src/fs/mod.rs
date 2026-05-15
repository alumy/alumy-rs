//! Filesystem utilities for path building and file-size handling.
//!
//! - [`path`]: Compile-time path construction via the [`build_path!`](crate::build_path) macro.
//! - [`filesize`]: Human-readable size parsing ([`filesize::parse_size`]) and formatting ([`filesize::format_size`]).

pub mod path;
pub mod filesize;
