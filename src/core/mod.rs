//! Core no_std-friendly building blocks for Alumy.
//!
//! This module contains utilities that can be shared by Linux, desktop, and MCU
//! targets without pulling in operating-system integrations.

pub mod version;

/// Filesystem-adjacent helpers that do not require OS access.
pub mod fs {
    /// Human-readable file-size parsing and formatting.
    pub mod filesize;
    /// Compile-time path construction utilities.
    pub mod path;
}
