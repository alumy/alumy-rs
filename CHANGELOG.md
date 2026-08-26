# Changelog

All notable changes to `alumy` are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and releases follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

No changes yet.

## [0.1.15] - 2026-08-26

### Added

- Unified `release_check.sh` for library, test, example, firmware, docs, and
  crates.io dry-run validation.
- Shared root skill reused by Codex and Claude.

### Changed

- Reorganized shared integration tests under `tests/core`.
- Simplified README and release documentation.

### Removed

- Unused `cortex-m` dependency from the FreeRTOS example.

## [0.1.14] - 2026-08-26

### Added

- Feature-gated Linux, bare-metal, FreeRTOS, and Embassy namespaces.
- Cross-platform integration-test entry point at `tests/platform.rs`.
- Shared integration tests under `tests/core`.
- Flashable Cortex-M example binaries for bare-metal, FreeRTOS, and Embassy.
- `release_check.sh` for release validation and crates.io dry-run checks.

### Changed

- Consolidated the publishable surface into the root `alumy` crate.
- Moved shared implementation into `src/core` and Linux implementation into
  `src/linux`.
- Organized platform tests and examples by target.
- Limited MCU-only dependencies to ARM targets.

### Removed

- Separate platform crate packages and obsolete test entry points.
- Unused `cortex-m` dependency from the FreeRTOS example.

[Unreleased]: https://github.com/alumy/alumy-rs/compare/v0.1.15...HEAD
[0.1.15]: https://github.com/alumy/alumy-rs/releases/tag/v0.1.15
[0.1.14]: https://github.com/alumy/alumy-rs/releases/tag/v0.1.14
