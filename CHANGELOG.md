# Changelog

All notable changes to this project will be documented in this file.

## [0.9.0] - 2026-04-12

### Changed

- Updated CLI auth guidance to current Codex sign-in flow (`codex` first-run sign in with ChatGPT account or API key).
- Clarified in CLI output/help and README that ChatGPT/Codex plans and OpenAI API usage are separate offerings with separate billing.
- Updated `usage` behavior for API-key-only profiles so it no longer fails when ChatGPT plan claims are missing.

### Added

- Added auth mode detection (`chatgpt`, `api_key`, `chatgpt+api_key`, `unknown`) when saving profiles.
- Added auth mode visibility in `codexctl list --detailed`.
- Added tests for auth mode detection.

## [0.8.0] - 2026-04-12

### Fixed

- Preserved file permissions and byte-level content during profile save/load paths to avoid cross-platform corruption.
- Stopped rewriting copied profile files during `save`; now only `auth.json` (when encrypted) and `profile.json` are written.
- Preserved file modes when decrypting `auth.json` into staging during profile load.
- Changed profile switching (`load` and `run`) to replace only `auth.json`, preserving local `Codex` sessions/history/state files.
- Enabled tar permission preservation during profile import.
- Fixed import path validation to accept safe `./` tar entries while still rejecting path traversal.

### Added

- Added atomic write helper for safe file replacement while preserving existing permissions.
- Added regression tests for byte fidelity (including CRLF/non-UTF8 payloads) and Unix permission preservation.

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
