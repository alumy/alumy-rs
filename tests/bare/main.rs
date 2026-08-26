#![cfg(feature = "bare")]
//! Skeleton integration tests for the bare platform layer.
//!
//! `bare` currently only forwards the shared `core` helpers (see
//! AGENTS.md), so these tests check that the re-export wiring in
//! `src/bare` stays intact rather than asserting platform-specific
//! behavior.
//!
//! As real platform APIs land in `alumy::bare`, extend this file
//! instead of adding a new one.

use alumy::bare::{build_path, crate_name, crate_version, fs, version};

#[test]
fn re_exports_build_path() {
    const PATH: &str = build_path!("/etc", "alumy", ".conf");
    assert_eq!(PATH, "/etc/alumy/alumy.conf");
}

#[test]
fn re_exports_crate_metadata_macros() {
    assert_eq!(crate_name!(), "alumy");
    assert!(!crate_version!().is_empty());
}

#[test]
fn re_exports_version_functions() {
    assert_eq!(version::name(), "alumy");
    assert_eq!(version::version(), crate_version!());
}

#[test]
fn re_exports_fs_filesize() {
    assert_eq!(fs::filesize::parse_size("4KiB"), Some(4096));
}

// TODO: replace with real bare-metal assertions once `alumy::bare`
// grows platform-specific submodules (e.g. boot, memory, interrupts).
