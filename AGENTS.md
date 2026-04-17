# Repository Guidelines

## Project Structure

This is a Rust CLI crate that exposes the Flow Engineering REST API as the `flow` binary.

```
src/
  main.rs              entry point — parses CLI, routes to handlers
  lib.rs               library target exposing all modules (required for integration tests)
  output.rs            OutputFormat enum (Json/Table) and print_output()
  client.rs            FlowClient, HttpSend trait, Auth enum
  config.rs            Config struct, config file loading/saving
  cli/
    mod.rs             Cli struct, Commands enum, shared arg structs
    auth.rs            AuthCommands
    config.rs          ConfigCommands
    configurations.rs  ConfigurationCommands
    documents.rs       DocumentCommands
    interfaces.rs      InterfaceCommands
    members.rs         MemberCommands
    orgs.rs            OrgsCommands
    projects.rs        ProjectCommands
    requirements.rs    RequirementCommands (and all requirement-specific arg structs)
    systems.rs         SystemCommands
    test_cases.rs      TestCaseCommands
    test_cycles.rs     TestCycleCommands
    test_plans.rs      TestPlanCommands
    test_runs.rs       TestRunCommands
    values.rs          ValueCommands
    util.rs            UtilCommands
  handlers/
    mod.rs             shared helpers (resolve_context, list_query, patch_collection, etc.)
                       and pub use re-exports of all handle_* functions
    auth.rs … util.rs  one file per resource, matching src/cli/ layout
tests/
  unit.rs              unit test binary entry point (cargo test --test unit)
  unit/
    helpers.rs         MockHttpClient implementing HttpSend
    output.rs          output formatting tests
    orgs.rs … …        handler unit tests per resource
  integration.rs       integration test binary entry point (cargo test --test integration)
  integration/
    requirements.rs … integration smoke tests per resource (skip without credentials)
docs/
  superpowers/
    specs/             design specs
    plans/             implementation plans
.github/
  workflows/
    ci.yml             lint + unit tests on every push/PR; integration tests on main with credentials
    release.yml        builds Linux/macOS/Windows binaries on v* tags, publishes GitHub Release
```

## Build, Test, and Development Commands

```bash
cargo build                    # compile dev binary to target/debug/flow
cargo check                    # fast compile validation
cargo fmt --all                # format all Rust code
cargo clippy -- -D warnings    # lint (fails on any warning)
cargo test --test unit         # unit tests only (no credentials needed)
cargo test --test integration  # integration tests (requires FLOW_ACCESS_TOKEN, FLOW_ORG, FLOW_PROJECT)
cargo test                     # all tests
cargo run -- --help            # inspect the CLI surface locally
cargo run -- auth status       # verify config loading without an API call
```

Run before opening a PR:

```bash
cargo fmt --all && cargo clippy -- -D warnings && cargo test --test unit
```

## Coding Style & Naming Conventions

Standard Rust style, formatted with `cargo fmt`. Follow Rust naming conventions:

- `snake_case` for functions, variables, modules
- `PascalCase` for structs and enums
- `SCREAMING_SNAKE_CASE` for constants

**Handler pattern:** each `handle_X` function is `pub async fn handle_X<C: HttpSend>(command: XCommands, client: &C, config: &Config, output: OutputFormat) -> Result<()>`. Shared helpers (`resolve_context`, `list_query`, `load_json_payload`, etc.) live in `handlers/mod.rs` as `pub(crate)` functions.

**CLI pattern:** each resource's command enum and its arg structs live in `src/cli/{resource}.rs`. Shared arg structs (`ResourceContextArgs`, `ListArgs`, `PatchCollectionArgs`, `JsonPayloadArgs`, etc.) live in `src/cli/mod.rs`.

## Testing Guidelines

**Unit tests** (`tests/unit/`) use `MockHttpClient` from `tests/unit/helpers.rs`. Tests assert on the HTTP method and path the handler constructs. Add a new test file under `tests/unit/` for each handler, declared in `tests/unit.rs` with `#[path = "unit/{name}.rs"] mod {name};`.

**Integration tests** (`tests/integration/`) call the real API. Guard every test:

```rust
let Some((token, org, project)) = require_credentials() else { return };
```

`require_credentials()` is defined in `tests/integration.rs` and returns `None` if `FLOW_ACCESS_TOKEN`, `FLOW_ORG`, or `FLOW_PROJECT` are not set. On CI, the workflow hard-fails if these secrets are absent before running the integration binary.

Name tests for behaviour: `requirements_list_returns_without_error`, `list_org_calls_get_on_org_members_path`.

## Commit & Pull Request Guidelines

Use clear imperative commit messages:

- `feat: add members command`
- `refactor: split handlers.rs into per-resource modules`
- `ci: add GitHub Actions CI workflow`

Pull requests should include a short summary, any API or config impact, the verification steps run, and example CLI usage if behaviour changed.

## Security & Configuration Tips

Do not commit live Flow credentials or local config files. Prefer environment variables for sensitive values. The config file (`~/.config/flow-cli/config.json`) is local-only state and should be in `.gitignore`.
