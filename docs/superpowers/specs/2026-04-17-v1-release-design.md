# flow-cli v1.0.0 Release Design

**Date:** 2026-04-17
**Scope:** Full API coverage, module refactor, output formatting, test suite, binary rename

---

## Goals

Ship a v1.0.0 release of `flow-cli` that:
- Covers every endpoint in the Flow Engineering REST API v1
- Fixes deprecated endpoint usage
- Refactors the codebase into maintainable per-resource modules
- Adds human-readable table output alongside existing JSON output
- Ships with a unit + integration test suite
- Renames the binary from `flow-cli` to `flow`

---

## Approach

Work **resource-by-resource** (Option B). For each resource, complete all four concerns before moving to the next: move to module → add missing endpoints → add table output → add tests. This keeps each unit of work reviewable and releasable independently.

---

## Module Structure

```
src/
├── main.rs
├── cli/
│   ├── mod.rs              (root Cli struct, top-level Commands enum)
│   ├── auth.rs
│   ├── config.rs
│   ├── configurations.rs   (new)
│   ├── documents.rs        (new)
│   ├── interfaces.rs       (new)
│   ├── members.rs          (new)
│   ├── orgs.rs
│   ├── projects.rs
│   ├── requirements.rs
│   ├── systems.rs
│   ├── test_cases.rs
│   ├── test_cycles.rs      (new)
│   ├── test_plans.rs
│   ├── test_runs.rs        (new)
│   ├── values.rs
│   └── util.rs
├── handlers/
│   ├── mod.rs              (dispatch only — matches Commands and calls per-resource handler)
│   ├── auth.rs
│   ├── config.rs
│   ├── configurations.rs   (new)
│   ├── documents.rs        (new)
│   ├── interfaces.rs       (new)
│   ├── members.rs          (new)
│   ├── orgs.rs
│   ├── projects.rs
│   ├── requirements.rs
│   ├── systems.rs
│   ├── test_cases.rs
│   ├── test_cycles.rs      (new)
│   ├── test_plans.rs
│   ├── test_runs.rs        (new)
│   ├── values.rs
│   └── util.rs
├── client.rs               (unchanged, gains HttpSend trait)
├── config.rs               (unchanged)
└── output.rs               (new — shared formatting logic)

tests/
├── unit/
│   ├── mod.rs
│   ├── handlers/
│   │   ├── auth.rs
│   │   ├── configurations.rs
│   │   ├── documents.rs
│   │   ├── interfaces.rs
│   │   ├── members.rs
│   │   ├── orgs.rs
│   │   ├── projects.rs
│   │   ├── requirements.rs
│   │   ├── systems.rs
│   │   ├── test_cases.rs
│   │   ├── test_cycles.rs
│   │   ├── test_plans.rs
│   │   ├── test_runs.rs
│   │   └── values.rs
│   └── output.rs
└── integration/
    ├── mod.rs              (skips all if FLOW_ACCESS_TOKEN not set)
    ├── configurations.rs
    ├── documents.rs
    ├── interfaces.rs
    ├── members.rs
    ├── orgs.rs
    ├── projects.rs
    ├── requirements.rs
    ├── systems.rs
    ├── test_cases.rs
    ├── test_cycles.rs
    ├── test_plans.rs
    ├── test_runs.rs
    └── values.rs
```

---

## Output Formatting

A global `--output` flag is added to the root `Cli` struct, defaulting to `json` for backwards compatibility:

```
flow --output table requirements list
flow --output json requirements list   # same as default
```

`output.rs` exposes:

```rust
pub enum OutputFormat { Json, Table }

pub fn print_output(value: &Value, format: OutputFormat) -> Result<()>
```

- **JSON mode**: existing `serde_json::to_string_pretty` behaviour, unchanged.
- **Table mode**: renders an ASCII table. Array responses: each item is a row, top-level keys are column headers. Object responses: two-column key/value layout. Implemented with standard string formatting — no external table crate.

Handlers continue to build a `serde_json::Value` internally, then call `print_output` with the format passed from CLI args. `OutputFormat` is threaded from `main.rs` into each handler.

---

## New Resources & Commands

### New top-level commands

| Command | Subcommands |
|---|---|
| `documents` | `list`, `get`, `create`, `patch`, `delete`, `set-import-id` |
| `interfaces` | `list`, `create`, `patch`, `delete` |
| `members` | `list-org`, `add-org`, `remove-org`, `list-project`, `add-project`, `remove-project` |
| `configurations` | `list`, `create` |
| `test-cycles` | `get`, `delete`, `create` (requires `--test-plan-id`) |
| `test-runs` | `get`, `create` (requires `--test-case-id`), `patch`, `delete`, `set-steps` (`PUT /testCycle/{id}/testRun/{id}/steps`) |

All new commands accept `--org` / `--project` context args (env: `FLOW_ORG`, `FLOW_PROJECT`) consistent with existing resources.

### New subcommands on existing resources

**requirements**
- `filter` — `POST /requirements/filter`
- `set-stage` — `PUT /requirements/stage`
- `set-import-id` — `PUT /requirements/importid`
- `set-value` — `PUT /requirements/value` (replaces deprecated `PUT /requirement/{id}/value`)
- `list-test-cases` — `GET /requirement/{id}/testCases`
- `list-test-plans` — `GET /requirement/{id}/testPlans`
- `upload-file` — `POST /requirement/{id}/uploadFile`
- `upload-image` — `POST /requirement/{id}/imageUrl/{fileId}`
- `link-jira` — `POST /requirement/{id}/jiraIssues`
- `unlink-jira` — `DELETE /requirement/{id}/jiraIssues/{jiraIssueId}`
- `link` — `POST /requirement/{id}/links/{linkType}`
- `unlink` — `DELETE /requirement/{id}/links/{linkType}/{linkedId}`
- `unlink-cross-project` — `DELETE /requirement/{id}/links/{linkType}/cross_project/{projectAlias}/{linkedId}`
- `link-test-case` — `PUT /link/requirementTestCase`
- `link-test-case-cross-project` — `PUT /link/requirementTestCase/crossProject`
- `get-custom-fields` — `GET /requirements/customFields`
- `patch-custom-fields` — `PATCH /requirements/customFields`
- `rename-custom-field-option` — `POST /requirements/customFields/renameOption`
- `add-configuration` / `remove-configuration` — `POST|DELETE /requirements/configuration`
- `list` gains `--scope org|project|without-system` flag

**systems**
- `bulk-update` — `PUT /systems`
- `link-document` / `list-documents` — `POST|GET /system/{id}/links/documents`
- `link-requirement` / `list-requirements` / `unlink-requirement` — `POST|GET|DELETE /system/{id}/links/requirements`
- `link-test-case` / `list-test-cases` / `unlink-test-case` — `POST|GET|DELETE /system/{id}/links/testCases`
- `link-test-plan` / `list-test-plans` — `POST|GET /system/{id}/links/testPlans`
- `rename-custom-field-option` — `POST /systems/customFields/renameOption`

**test-cases**
- `set-steps` — `PUT /testCase/{id}/steps`
- `set-import-id` — `PUT /testCases/importid`
- `upload-file` — `POST /testCase/{id}/file/{fileId}`
- `link-jira` / `unlink-jira` — `POST|DELETE /testCase/{id}/jiraIssues`
- `list-requirements` — `GET /testCase/{id}/links/requirements`
- `get-custom-fields` / `patch-custom-fields` — `GET|PATCH /testCases/customFields`
- `rename-custom-field-option` — `POST /testCases/customFields/renameOption`
- `add-configuration` / `remove-configuration` — `POST|DELETE /testCases/configuration`
- `set-stages` — `PUT /testCaseStages`
- `create-test-run` — `POST /testCase/{id}/testRun`

**test-plans**
- `create` — `POST /testPlans`
- `get` — `GET /testPlan/{id}`
- `delete` — `DELETE /testPlan/{id}`
- `create-cycle` — `POST /testPlan/{id}/testCycle`
- `set-import-id` — `PUT /testPlans/importid`
- `link-test-case` — `PUT /link/testPlanTestCase`
- `link-test-case-cross-project` — `PUT /link/testPlanTestCase/crossProject`

**values**
- `get` — `GET /value/{id}`
- `set-import-id` — `PUT /values/importid`

### Deprecation fixes

- `systems list` migrates from `GET /systems` (deprecated) to `GET /systems/paged`
- `requirements set-value` uses `PUT /requirements/value`; the per-item `PUT /requirement/{id}/value` is removed

---

## Test Suite

### Unit tests (`tests/unit/`)

`FlowClient` gains an `HttpSend` trait:

```rust
pub trait HttpSend {
    async fn send(&self, method: Method, path: &str, query: &[(String, String)], body: Option<Value>, with_auth: bool) -> Result<Value>;
}
```

`FlowClient` implements `HttpSend`. Tests inject a `MockHttpClient` that returns pre-canned JSON. Each handler test file verifies:
- Correct URL path and HTTP method constructed
- Query params built correctly (pagination, filters, scopes)
- Output formatted correctly for both JSON and table modes
- Error paths (missing org/project, missing required flags, bad API response)

### Integration tests (`tests/integration/`)

`tests/integration/mod.rs` provides a `require_credentials()` helper — returns early if `FLOW_ACCESS_TOKEN`, `FLOW_ORG`, or `FLOW_PROJECT` are not set. Each test calls this at the top. Tests exercise the full CRUD lifecycle where the API supports it and clean up created resources in a `defer`-style pattern.

**Running tests:**
```bash
cargo test                                                        # unit only
FLOW_ACCESS_TOKEN=xxx FLOW_ORG=xxx FLOW_PROJECT=xxx cargo test   # all tests
```

---

## CI/CD (GitHub Actions)

Two workflows live in `.github/workflows/`.

### `ci.yml` — runs on every push and pull request to `main`

1. **Check** — `cargo check` to catch compile errors fast
2. **Lint** — `cargo clippy -- -D warnings` (fail on any warning)
3. **Format** — `cargo fmt --check`
4. **Unit tests** — `cargo test` (no credentials, always runs)
5. **Integration tests** — `cargo test` with `FLOW_ACCESS_TOKEN`, `FLOW_ORG`, and `FLOW_PROJECT` injected from GitHub repository secrets. Runs only on push to `main` (not on PRs from forks, which can't access secrets). If secrets are absent the test suite skips gracefully.

Matrix: runs on `ubuntu-latest`. Rust toolchain pinned to stable via `dtolnay/rust-toolchain@stable`.

### `release.yml` — runs on push of a `v*` tag (e.g. `v1.0.0`)

1. Builds release binaries for three targets:
   - `x86_64-unknown-linux-gnu` (runs on `ubuntu-latest`)
   - `aarch64-apple-darwin` (runs on `macos-latest`)
   - `x86_64-pc-windows-msvc` (runs on `windows-latest`)
2. Each target runs in its own matrix job on the appropriate native runner — no cross-compilation needed.
3. Linux/macOS binaries are stripped and archived as `flow-{target}.tar.gz`. The Windows binary is archived as `flow-x86_64-pc-windows-msvc.zip`.
4. Creates a GitHub Release via `softprops/action-gh-release` and uploads all four archives as release assets.

The release workflow requires a `GITHUB_TOKEN` (automatically provided by Actions) — no additional secrets needed for publishing.

---

## Versioning & Binary Name

- `Cargo.toml` version: `0.1.0` → `1.0.0`
- `[[bin]] name`: `flow-cli` → `flow`
- No other breaking changes: JSON default output, config file location, env var names, and all existing command names are unchanged.

---

## Implementation Order

Work through resources in this sequence, completing all four concerns (module split → new endpoints → table output → tests) before moving on:

1. Shared infrastructure: `output.rs`, `HttpSend` trait, test harness setup
2. Existing resources (refactor + fill gaps): auth, config, orgs, projects, requirements, systems, test-cases, test-plans, values, util
3. New resources: documents, interfaces, members, configurations, test-cycles, test-runs
4. CI/CD: `ci.yml` and `release.yml` GitHub Actions workflows
5. `Cargo.toml`: version bump + binary rename
