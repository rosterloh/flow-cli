# flow-cli v1.0.0 — Plan 4: CI/CD and Release

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add GitHub Actions CI (lint + unit tests on every push/PR, integration tests on main) and a release workflow that builds and publishes binaries for Linux x86, macOS ARM, and Windows x86 on version tags. Then bump to v1.0.0 and rename the binary from `flow-cli` to `flow`.

**Architecture:** Two workflow files in `.github/workflows/`. CI uses a single `ubuntu-latest` runner. Release uses a matrix of three native runners — one per target — so no cross-compilation is needed. Release assets are uploaded to a GitHub Release created by `softprops/action-gh-release`.

**Tech Stack:** GitHub Actions, `dtolnay/rust-toolchain`, `softprops/action-gh-release`.

**Prerequisite:** Plans 1, 2, and 3 must be complete (`cargo test` green).

---

### Task 18: CI workflow — `.github/workflows/ci.yml`

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Create `.github/workflows/` directory**

```bash
mkdir -p .github/workflows
```

- [ ] **Step 2: Create `.github/workflows/ci.yml`**

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  ci:
    name: Check, lint, format, test
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install stable Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache cargo registry and build
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-cargo-

      - name: Check (fast compile error detection)
        run: cargo check

      - name: Clippy (fail on warnings)
        run: cargo clippy -- -D warnings

      - name: Format check
        run: cargo fmt --check

      - name: Unit tests
        run: cargo test --test unit

      - name: Integration tests
        # Only runs on push to main; PRs from forks cannot access secrets.
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        env:
          FLOW_ACCESS_TOKEN: ${{ secrets.FLOW_ACCESS_TOKEN }}
          FLOW_ORG: ${{ secrets.FLOW_ORG }}
          FLOW_PROJECT: ${{ secrets.FLOW_PROJECT }}
        run: cargo test --test integration
```

- [ ] **Step 3: Verify the workflow file is valid YAML**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))" && echo "valid"
```

Expected: `valid`

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add GitHub Actions CI workflow"
```

---

### Task 19: Release workflow — `.github/workflows/release.yml`

**Files:**
- Create: `.github/workflows/release.yml`

- [ ] **Step 1: Create `.github/workflows/release.yml`**

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    permissions:
      contents: write

    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            archive: tar.gz
          - target: aarch64-apple-darwin
            os: macos-latest
            archive: tar.gz
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            archive: zip

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain for ${{ matrix.target }}
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Cache cargo registry and build
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-${{ matrix.target }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: ${{ runner.os }}-${{ matrix.target }}-cargo-

      - name: Build release binary
        run: cargo build --release --target ${{ matrix.target }}

      - name: Strip binary (Linux and macOS)
        if: matrix.os != 'windows-latest'
        run: strip target/${{ matrix.target }}/release/flow

      - name: Package binary (Linux and macOS)
        if: matrix.archive == 'tar.gz'
        run: |
          ARCHIVE=flow-${{ matrix.target }}.tar.gz
          tar -czf "$ARCHIVE" -C target/${{ matrix.target }}/release flow
          echo "ASSET=$ARCHIVE" >> $GITHUB_ENV

      - name: Package binary (Windows)
        if: matrix.archive == 'zip'
        shell: pwsh
        run: |
          $archive = "flow-${{ matrix.target }}.zip"
          Compress-Archive -Path "target\${{ matrix.target }}\release\flow.exe" -DestinationPath $archive
          echo "ASSET=$archive" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append

      - name: Upload release asset
        uses: softprops/action-gh-release@v2
        with:
          files: ${{ env.ASSET }}
          generate_release_notes: true
```

- [ ] **Step 2: Verify the workflow file is valid YAML**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))" && echo "valid"
```

Expected: `valid`

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add GitHub Actions release workflow for v* tags"
```

---

### Task 20: Version bump and binary rename

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/config.rs` (update error message referencing old binary name)

- [ ] **Step 1: Update `Cargo.toml`**

Replace the `[package]` section:

```toml
[package]
name = "flow-cli"
version = "1.0.0"
edition = "2024"

[[bin]]
name = "flow"
path = "src/main.rs"
```

- [ ] **Step 2: Update `src/config.rs` — remove old binary name from error message**

The `config_path()` function joins the config dir with `"flow-cli"`. This is the config directory name, not the binary — keep it as `flow-cli` so existing users' config files are found automatically. No change needed here.

- [ ] **Step 3: Verify the binary name in `src/cli/mod.rs` is already updated**

The `name = "flow"` in the `#[command(...)]` attribute was set in Plan 1, Task 4, Step 11. Confirm it reads `name = "flow"`:

```bash
grep 'name = "flow"' src/cli/mod.rs
```

Expected: one match.

- [ ] **Step 4: Run full test suite to confirm nothing broke**

```bash
cargo test
```

Expected: all tests pass.

- [ ] **Step 5: Build release binary and confirm the output name**

```bash
cargo build --release && ls -lh target/release/flow
```

Expected: `target/release/flow` exists (not `flow-cli`).

- [ ] **Step 6: Smoke test the binary**

```bash
./target/release/flow --help
```

Expected: help text shows `flow` as the command name and `1.0.0` as the version.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "release: bump version to 1.0.0 and rename binary to flow"
```

- [ ] **Step 8: Tag the release**

```bash
git tag v1.0.0
git push origin main --tags
```

This triggers the `release.yml` workflow, which builds and publishes binaries for Linux x86, macOS ARM, and Windows x86 as GitHub Release assets.

---

**Plan 4 complete. v1.0.0 is released.**

---

## Summary of all four plans

| Plan | Focus | Tasks |
|---|---|---|
| 1 — Infrastructure | output.rs, HttpSend trait, module split, --output flag | 6 |
| 2 — Existing resources | requirements, systems, test-cases, test-plans, values gaps | 5 |
| 3 — New resources | documents, interfaces, members, configurations, test-cycles, test-runs | 6 |
| 4 — CI/CD + release | ci.yml, release.yml, v1.0.0 tag | 3 |

Execute plans in sequence — each plan's output is the prerequisite for the next.
