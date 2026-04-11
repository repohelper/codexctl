# Changelog

All notable changes to this project will be documented in this file.

## [0.7.0] - 2026-04-11

### Changed

- Optimized CI execution by removing duplicated cross-target packaging from regular CI while keeping full release matrix builds.
- Optimized Docker build/runtime layers and release trigger behavior for faster pipeline runs and lower image overhead.
- Simplified Dependabot npm scope to the wrapper package (`/npm`) to avoid repeated scans on platform-only packages.
- Consolidated duplicated utility test modules by removing mirrored `*_test.rs` files and relying on in-module tests.

### Security

- Added signed release blobs via keyless `cosign` in release workflow.
- Added generated `SHA256SUMS` and SPDX SBOM release assets for each tagged release.
- Added GitHub provenance attestation and SBOM attestation steps for release artifacts.

## [0.6.3] - 2026-04-11

### Changed

- Reframed product messaging around Codex Controller for Codex CLI.
- Aligned command/help/docs examples around the active `codexctl` command surface.
- Updated Rust and npm dependency sets and lockfile.
- Updated npm wrapper and platform package versions to `0.6.3`.

### Security

- Removed `age` `ssh` feature dependency path to reduce transitive crypto exposure.
- Added GitHub security automation:
  - Rust advisory scan
  - npm audit scan
  - secret scanning
  - CodeQL
  - dependency review on PRs
- Added repository security policy and community governance docs.
