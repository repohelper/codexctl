# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Added `codexctl shapeup init-bet` to scaffold repo-local shaped bet specs under `.codexctl/tasks/`.
- Added `codexctl shapeup lint` to enforce the current Shape Up + DDD bet model, including bounded-context checks and placeholder detection.

### Changed

- Added README coverage for the Shape Up bet authoring and unattended loop workflow.
- Relaxed initial `run-loop` dirty-repo blocking so repo-local `.codexctl/` planning changes do not block unattended starts.
- Added a timeout-backed failure path for `run-loop --notify-cmd`; stalled notify hooks are now recorded as `notify_failed` instead of hanging the run.
- Removed the unused `aes-gcm` release-candidate dependency and kept profile encryption on the stable `age` stack already used by the codebase.
- Refreshed the Rust lockfile to current compatible transitive versions.
- Updated release automation to the current `softprops/action-gh-release@v3` action and aligned the npm audit workflow to Node.js 24.

## [0.10.0] - 2026-04-13

### Changed

- Made `status`, `usage`, `verify`, `doctor`, and `setup` auth-mode aware so ChatGPT/Codex, API-key, and hybrid profiles describe the capabilities they actually have.
- Updated `verify` and `usage --all` to treat API-key-only profiles as first-class valid profiles and to report encrypted profiles as locked instead of invalid.
- Changed `load` to back up only the live `auth.json`, matching the current auth-only switching model.
- Reduced default logging noise from startup info logs to warnings unless `--verbose` or `RUST_LOG` is set.

### Added

- Added `--json` output to `status`, `usage`, `verify`, and `doctor` for scripting and CI.
- Added `codexctl run --passphrase` support so encrypted profiles work in one-shot automation flows.
- Added command-level integration tests covering JSON output for `status`, `usage --all`, and `verify`.

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
