# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-04-22

### Added

- Flag-mode shortcuts for single-item mutation commands. Users no longer need
  to hand-craft the server's JSON payloads for common operations — named
  flags build the request body automatically. `--json` / `--body-file`
  continue to work unchanged for batch and custom-field payloads.
  - `test-cases patch --id N --name ... --description ... --owner ...`
  - `test-plans patch --id N --name ... --description ...`
  - `requirements patch --id N --name ... --owner ...`
  - `test-cases set-steps --id N --steps-file path.json` accepts a plain
    `[{"action","expected"}]` array and wraps it into the server's nested
    shape. The server generates `caseStepId` values.
  - `requirements link-test-case --requirement-id R --test-case-id T`
  - `test-plans link-test-case --test-plan-id P --test-case-id T`
  - `systems link-test-plan --id <uuid> --test-plan-id P`
  - `systems link-test-case --id <uuid> --test-case-id T`
  - `systems link-requirement --id <uuid> --requirement-id R`
  - `systems link-document --id <uuid> --document-id D`
- `docs/cli-improvements-backlog.md` tracking remaining ergonomics work.

### Fixed

- `tests/integration/systems.rs` no longer fails to compile after the
  earlier `ListSystemsArgs` split.
- `requirements link-test-case-cross-project` no longer exposes
  `--requirement-id` / `--test-case-id` in its `--help`; the flags are
  only relevant to the in-project variant.

### Internal

- Three shared payload-builder helpers added to `src/handlers/mod.rs`:
  `build_patch_single`, `build_links_wrapper`, `build_system_link_item`.
  All marked `#[doc(hidden)] pub` — reachable from the test crate without
  widening the library's public Rust API.
- Removed dead `SystemLinkPayloadArgs` struct; each `systems link-*` variant
  now has its own purpose-specific args struct.
- `tempfile` added to `[dev-dependencies]` for steps-file tests.
- Unit test count grew from 51 to 82.

## [1.0.0] - 2026-04-17

Initial release — full Flow Engineering REST API v1 coverage, per-resource
module structure, JSON and table output, unit + integration test suites.
Binary renamed from `flow-cli` to `flow`.
