# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2021-02-13

### Added

- a Dependabot configuration to help me keep the dependencies up to date

- a "Release" workflow for automating builds and deployments to crates.io

### Changed

- **Breaking**: The crate now uses Rust 2018 idioms.
  In particular, `extern crate` lines were removed.

- All pull requests against the default branch are now only _checked_ (via `cargo clippy` and friends) on `stable` and `beta` Rust.
  This is because of a number of cases where `nightly` Rust broke a tool, which caused our builds to fail.
  **We still are _testing_ the code against `nightly`**, since this only requires that the compiler work.

[Unreleased]: https://github.com/rye/git-heckout/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/rye/git-heckout/compare/v0.5.1...v0.6.0

