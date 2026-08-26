#![cfg(feature = "freertos")]
//! Skeleton integration tests for the FreeRTOS platform layer.
//!
//! `freertos` currently only forwards the shared `core` helpers (see
//! AGENTS.md), so these tests check that the re-export wiring in
//! `src/freertos` stays intact rather than asserting platform-specific
//! behavior.
//!
//! As real platform APIs land in `alumy::freertos`, extend this file
//! instead of adding a new one.

use alumy::freertos::{build_path, crate_name, crate_version, fs, version};

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
    assert_eq!(fs::filesize::parse_size("8KiB"), Some(8192));
}

// TODO: replace with real FreeRTOS-task assertions once
// `alumy::freertos` grows platform-specific submodules
// (e.g. task, queue, semaphore).
