# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-03

### Changed
- **BREAKING**: Updated Rust edition from 2018 to 2021
- **BREAKING**: Migrated from deprecated `failure` crate to `thiserror` for error handling
- **BREAKING**: Updated `reqwest` from 0.9 to 0.12 (still supports blocking API)
- Updated `serde` and `serde_json` to latest versions
- Set MSRV (Minimum Supported Rust Version) to 1.70
- Improved crate metadata for crates.io publishing

### Added
- Comprehensive CHANGELOG
- CI/CD workflows for automated testing
- Additional documentation and examples
- Better error types with more context

### Fixed
- Resolved all clippy warnings
- Fixed deprecated API usage
- Improved throttling correctness and determinism

### Maintenance
- Modernized codebase to 2021 Rust standards
- Added comprehensive test suite
- Improved documentation

---

## [0.0.6] - ~2019 (Original upstream)

Last version from original maintainer before fork.
