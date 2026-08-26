#![no_std]
#![warn(missing_docs)]
//! Core no_std-friendly building blocks for Alumy.
//!
//! This crate contains utilities that can be shared by Linux, desktop, and MCU
//! targets without pulling in operating-system integrations.

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod version;

/// Filesystem-adjacent helpers that do not require OS access.
pub mod fs {
    /// Human-readable file-size parsing and formatting.
    pub mod filesize;
    /// Compile-time path construction utilities.
    pub mod path;
}
