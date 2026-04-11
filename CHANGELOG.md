# Changelog

All notable changes to this project will be documented in this file.

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
