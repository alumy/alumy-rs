//! Skeleton integration tests for the MCU platform layers.
//!
//! `bare`, `freertos`, and `embassy` currently only forward the shared
//! `core` helpers (see AGENTS.md), so these tests check that the re-export
//! wiring in `src/bare`, `src/freertos`, and `src/embassy` stays intact
//! rather than asserting platform-specific behavior.
//!
//! As real platform APIs land in each module, extend the matching `mod`
//! block below instead of adding new files.

#[cfg(feature = "bare")]
mod bare_platform {
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
}

#[cfg(feature = "freertos")]
mod freertos_platform {
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
}

#[cfg(feature = "embassy")]
mod embassy_platform {
    use alumy::embassy::{build_path, crate_name, crate_version, fs, version};

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
        assert_eq!(fs::filesize::parse_size("16KiB"), Some(16384));
    }

    // TODO: replace with real Embassy-executor assertions once
    // `alumy::embassy` grows platform-specific submodules
    // (e.g. executor, task).
}
