# Repository Guidelines

## Project Structure & Module Organization

This repository is a small Rust CLI crate.

- `src/main.rs`: the entire application entrypoint, CLI definitions, config handling, and HTTP client logic.
- `Cargo.toml`: crate metadata and dependencies.
- `Cargo.lock`: locked dependency graph for reproducible builds.
- `README.md`: user-facing setup and usage documentation.

There are no separate test or asset directories yet. If the code grows, prefer extracting reusable logic from `src/main.rs` into focused modules under `src/` such as `auth.rs`, `client.rs`, or `commands/`.

## Build, Test, and Development Commands

- `cargo build`: compile the project.
- `cargo check`: fast compile validation without producing a release binary.
- `cargo fmt --all`: format the Rust codebase.
- `cargo test`: run automated tests.
- `cargo run -- --help`: inspect the CLI surface locally.
- `cargo run -- auth status`: verify config loading without making an authenticated API call.

Run these before opening a PR:

```bash
cargo fmt --all
cargo check
cargo test
```

## Coding Style & Naming Conventions

Use standard Rust style with 4-space indentation and rely on `cargo fmt` for formatting. Follow Rust naming conventions:

- `snake_case` for functions, variables, and modules
- `PascalCase` for structs and enums
- `SCREAMING_SNAKE_CASE` for constants

Keep command handlers small and prefer shared helper functions for request building, config lookup, and JSON parsing.

## Testing Guidelines

This repository does not yet have dedicated tests, but new logic should add them where practical. Prefer unit tests near the code under `src/` and integration tests under `tests/` when command behavior becomes more complex.

Name tests for behavior, for example `exchanges_refresh_token` or `loads_project_from_config`.

## Commit & Pull Request Guidelines

There is no existing Git history yet, so use clear imperative commit messages such as:

- `Add raw request command`
- `Document auth configuration`

Pull requests should include:

- a short summary of the change
- any API or config impact
- the verification steps you ran
- example CLI usage if behavior changed

## Security & Configuration Tips

Do not commit live Flow credentials or local config files. Prefer environment variables for sensitive values, and treat `~/.config/flow-cli/config.json` as local-only state.
