# CLI Ergonomics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add single-item flag shortcuts to mutation commands so users can run `flow test-cases patch --id N --owner X` instead of hand-crafting the server's awkward `[{"testCaseId": N, "owner": "X"}]` payload. `--json` / `--body-file` stays as-is.

**Architecture:** Three shared payload-builder functions in `src/handlers/mod.rs` (one per shape: bare-array-of-single-object, `{"links": [...]}` wrapper, system-link bare array). Each affected command gets (a) new optional flags on its args struct in `src/cli/*.rs` with clap mutex constraints against `--json`/`--body-file`, and (b) a handler branch in `src/handlers/*.rs` that picks between the flag-built payload and the raw JSON path.

**Tech Stack:** Rust 2024, `clap 4.5` (derive + `ArgGroup`), `serde_json`, existing `tokio`/`reqwest` stack. No new crates.

**Spec:** `docs/superpowers/specs/2026-04-22-cli-ergonomics-design.md`.

---

## File Structure

**Modified files:**

- `src/handlers/mod.rs` — add three payload-builder functions: `build_patch_single`, `build_links_wrapper`, `build_system_link_item`.
- `src/cli/test_cases.rs` — add flag fields on the Patch and SetSteps args structs.
- `src/cli/test_plans.rs` — add flag fields on Patch and LinkTestCase args structs.
- `src/cli/requirements.rs` — add flag fields on Patch and LinkTestCase args structs.
- `src/cli/systems.rs` — split `SystemLinkPayloadArgs` usage per variant (`--test-plan-id`, `--test-case-id`, `--requirement-id`, `--document-id`) OR add a single `--entity-id` flag; this plan uses per-variant flags, documented inline.
- `src/handlers/test_cases.rs`, `src/handlers/test_plans.rs`, `src/handlers/requirements.rs`, `src/handlers/systems.rs` — branch on flag-mode vs body-mode for the affected commands.

**New files:**

- `tests/unit/payload.rs` — direct unit tests for the three payload-builder functions.

**Test files modified** (add one test per new flag surface):

- `tests/unit/test_cases.rs`
- `tests/unit/test_plans.rs`
- `tests/unit/requirements.rs`
- `tests/unit/systems.rs`
- `tests/unit.rs` — register the new `payload` module.

**Documentation:**

- `README.md` — add a short "Flag-mode shortcuts" section under Examples.
- `.claude/skills/flow-cli/SKILL.md` — update the payload-shape section to show the new flag mode alongside JSON mode.

---

## Clap mutex convention (shared across tasks)

Every new flag (e.g. `--id`, `--owner`, `--name`, `--requirement-id`, `--test-case-id`, `--steps-file`) is declared with:

```rust
#[arg(long, conflicts_with_all = ["json", "body_file"])]
pub id: Option<i64>,
```

The existing `JsonPayloadArgs.json` and `body_file` already have `conflicts_with = "..."` between themselves; we extend the mutex outward by adding `conflicts_with_all` on the new flag args. Post-clap validation (flag mode needs at least one field flag; link commands need the partner id) lives in the handler via `anyhow::bail!`.

---

## Task 1: Add `build_patch_single` helper

**Files:**
- Modify: `src/handlers/mod.rs` (append to existing helpers around line 120–180)
- Modify: `tests/unit.rs` (register `payload` module)
- Create: `tests/unit/payload.rs`

- [ ] **Step 1: Write failing unit test for the builder**

Create `tests/unit/payload.rs`:

```rust
// tests/unit/payload.rs
use serde_json::{Value, json};

use flow_cli::handlers::build_patch_single;

#[test]
fn build_patch_single_wraps_id_and_fields_in_array() {
    let fields = vec![
        ("owner".to_string(), json!("rio@skl.vc")),
        ("name".to_string(), json!("New Name")),
    ];
    let body: Value = build_patch_single("testCaseId", json!(326), fields);
    assert_eq!(
        body,
        json!([{ "testCaseId": 326, "owner": "rio@skl.vc", "name": "New Name" }])
    );
}

#[test]
fn build_patch_single_skips_no_fields() {
    let body: Value = build_patch_single("testCaseId", json!(326), vec![]);
    assert_eq!(body, json!([{ "testCaseId": 326 }]));
}
```

Register module. Modify `tests/unit.rs`, add before the `mod requirements` line:

```rust
mod payload;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit payload::`
Expected: fails to compile — `build_patch_single` not found.

- [ ] **Step 3: Implement the builder**

In `src/handlers/mod.rs`, add (after the existing `named_items_body` function around line 128):

```rust
/// Build a single-item patch payload: `[{<idKey>: <idValue>, ...fields}]`.
/// Used by `{resource} patch` commands where the server expects an array
/// of patch objects with the id nested inside.
pub fn build_patch_single(
    id_key: &str,
    id_value: Value,
    fields: Vec<(String, Value)>,
) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert(id_key.to_string(), id_value);
    for (k, v) in fields {
        obj.insert(k, v);
    }
    Value::Array(vec![Value::Object(obj)])
}
```

Make sure `build_patch_single` is `pub` so the unit test crate can call it.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test unit payload::`
Expected: 2 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/handlers/mod.rs tests/unit.rs tests/unit/payload.rs
git commit -m "feat(handlers): add build_patch_single payload helper"
```

---

## Task 2: Add `build_links_wrapper` helper

**Files:**
- Modify: `src/handlers/mod.rs`
- Modify: `tests/unit/payload.rs`

- [ ] **Step 1: Write failing unit test**

Append to `tests/unit/payload.rs`:

```rust
use flow_cli::handlers::build_links_wrapper;

#[test]
fn build_links_wrapper_wraps_single_link_in_links_array() {
    let link = json!({ "requirementId": 2855, "testCaseId": 326 });
    let body = build_links_wrapper(vec![link]);
    assert_eq!(
        body,
        json!({ "links": [{ "requirementId": 2855, "testCaseId": 326 }] })
    );
}

#[test]
fn build_links_wrapper_accepts_empty_links() {
    let body = build_links_wrapper(vec![]);
    assert_eq!(body, json!({ "links": [] }));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit payload::build_links_wrapper`
Expected: compilation error — function missing.

- [ ] **Step 3: Implement the helper**

In `src/handlers/mod.rs` (just below `build_patch_single`):

```rust
/// Wrap links into `{"links": [...]}` — used by the two cross-resource
/// link endpoints (`link/requirementTestCase`, `link/testPlanTestCase`).
pub fn build_links_wrapper(links: Vec<Value>) -> Value {
    json!({ "links": links })
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test unit payload::build_links_wrapper`
Expected: 2 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/handlers/mod.rs tests/unit/payload.rs
git commit -m "feat(handlers): add build_links_wrapper payload helper"
```

---

## Task 3: Add `build_system_link_item` helper

**Files:**
- Modify: `src/handlers/mod.rs`
- Modify: `tests/unit/payload.rs`

- [ ] **Step 1: Write failing unit test**

Append to `tests/unit/payload.rs`:

```rust
use flow_cli::handlers::build_system_link_item;

#[test]
fn build_system_link_item_wraps_entity_in_array() {
    let body = build_system_link_item("testPlanId", json!(203));
    assert_eq!(body, json!([{ "testPlanId": 203 }]));
}

#[test]
fn build_system_link_item_supports_string_ids() {
    let body = build_system_link_item("documentId", json!("doc-uuid"));
    assert_eq!(body, json!([{ "documentId": "doc-uuid" }]));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit payload::build_system_link_item`
Expected: compilation error.

- [ ] **Step 3: Implement the helper**

In `src/handlers/mod.rs`:

```rust
/// Build a single-item system-link payload: `[{<entityKey>: <entityValue>}]`.
/// Used by `systems link-*` commands where the server expects a bare array.
pub fn build_system_link_item(entity_key: &str, entity_value: Value) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert(entity_key.to_string(), entity_value);
    Value::Array(vec![Value::Object(obj)])
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test unit payload::`
Expected: 6 tests pass (all three helpers now covered).

- [ ] **Step 5: Commit**

```bash
git add src/handlers/mod.rs tests/unit/payload.rs
git commit -m "feat(handlers): add build_system_link_item payload helper"
```

---

## Task 4: `test-cases patch` flag mode

**Files:**
- Modify: `src/cli/test_cases.rs` (replace `TestCaseCommands::Patch` arg type)
- Modify: `src/cli/mod.rs` (re-export new arg type if needed)
- Modify: `src/handlers/test_cases.rs` (branch on flag vs JSON)
- Modify: `tests/unit/test_cases.rs` (add flag-mode tests)

- [ ] **Step 1: Write failing unit tests for flag-mode patch**

Append to `tests/unit/test_cases.rs`:

```rust
use flow_cli::cli::test_cases::TestCasePatchArgs;

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: Some(326),
            name: None,
            description: None,
            owner: Some("rio@skl.vc".into()),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PATCH");
    assert_eq!(call.path, "/org/o/project/p/testCases");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testCaseId": 326, "owner": "rio@skl.vc" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: Some(326),
            name: None,
            description: None,
            owner: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("at least one field flag"));
    assert!(mock.calls().is_empty());
}

#[tokio::test]
async fn patch_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: None,
            name: None,
            description: None,
            owner: None,
            payload: JsonPayloadArgs {
                json: Some(r#"[{"testCaseId": 1, "name": "x"}]"#.into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.body.as_ref().unwrap(), &json!([{ "testCaseId": 1, "name": "x" }]));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit test_cases::patch_`
Expected: compile error — `TestCasePatchArgs` not found.

- [ ] **Step 3: Replace `PatchCollectionArgs` with `TestCasePatchArgs` for this command**

In `src/cli/test_cases.rs`:

- Change the `Patch` variant:

```rust
Patch(TestCasePatchArgs),
```

- Add the new args struct at the bottom:

```rust
#[derive(Args, Debug)]
pub struct TestCasePatchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub name: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub description: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub owner: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- Update the `use` statement at the top to include any imports needed for the struct (e.g. `JsonPayloadArgs`, `ResourceContextArgs`).
- Export `TestCasePatchArgs` from `src/cli/mod.rs`: add to the `pub use test_cases::{...}` line.

- [ ] **Step 4: Update the handler to branch on mode**

In `src/handlers/test_cases.rs`, replace the existing `TestCaseCommands::Patch(args)` arm with:

```rust
TestCaseCommands::Patch(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(id) = args.id {
        let mut fields = Vec::new();
        if let Some(name) = args.name { fields.push(("name".to_string(), json!(name))); }
        if let Some(description) = args.description {
            fields.push(("description".to_string(), json!(description)));
        }
        if let Some(owner) = args.owner { fields.push(("owner".to_string(), json!(owner))); }
        if fields.is_empty() {
            anyhow::bail!(
                "at least one field flag required (--name, --description, --owner)"
            );
        }
        build_patch_single("testCaseId", json!(id), fields)
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!("/org/{org}/project/{project}/testCases");
    let response = client.send(Method::PATCH, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Update the imports at the top of the file:

```rust
use serde_json::json;
use super::{build_patch_single, list_query, load_json_payload, named_items_body, resolve_context};
```

(Drop `patch_collection` from the import list since it's no longer used for `Patch`.)

- [ ] **Step 5: Run the tests to verify they pass, then regression-check**

Run: `cargo test --test unit test_cases::`
Expected: all test_cases unit tests pass, including the three new ones.

Run the full suite: `cargo test --test unit`
Expected: no regressions.

- [ ] **Step 6: Commit**

```bash
git add src/cli/test_cases.rs src/cli/mod.rs src/handlers/test_cases.rs tests/unit/test_cases.rs
git commit -m "feat(test-cases): add flag-mode patch with --id/--name/--description/--owner"
```

---

## Task 5: `test-plans patch` flag mode

**Files:**
- Modify: `src/cli/test_plans.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/test_plans.rs`
- Modify: `tests/unit/test_plans.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/test_plans.rs`:

```rust
use flow_cli::cli::test_plans::TestPlanPatchArgs;

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_plans(
        TestPlanCommands::Patch(TestPlanPatchArgs {
            context: ctx("o", "p"),
            id: Some(202),
            name: Some("Internal Network Performance Test".into()),
            description: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PATCH");
    assert_eq!(call.path, "/org/o/project/p/testPlans");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testPlanId": 202, "name": "Internal Network Performance Test" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_plans(
        TestPlanCommands::Patch(TestPlanPatchArgs {
            context: ctx("o", "p"),
            id: Some(202),
            name: None,
            description: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("at least one field flag"));
}
```

Ensure the `use` statements at the top of `tests/unit/test_plans.rs` import `TestPlanCommands`, `handle_test_plans`, `JsonPayloadArgs`, `MockHttpClient`, `Config`, `OutputFormat`, and `json` — follow the existing pattern in `tests/unit/test_cases.rs`.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit test_plans::patch_`
Expected: compile error — `TestPlanPatchArgs` missing.

- [ ] **Step 3: Add `TestPlanPatchArgs` and wire the CLI**

In `src/cli/test_plans.rs`:

- Change the `Patch` variant to `Patch(TestPlanPatchArgs)`.
- Add the struct:

```rust
#[derive(Args, Debug)]
pub struct TestPlanPatchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub name: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub description: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export `TestPlanPatchArgs` in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

In `src/handlers/test_plans.rs`, replace the `TestPlanCommands::Patch` arm:

```rust
TestPlanCommands::Patch(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(id) = args.id {
        let mut fields = Vec::new();
        if let Some(name) = args.name { fields.push(("name".to_string(), json!(name))); }
        if let Some(description) = args.description {
            fields.push(("description".to_string(), json!(description)));
        }
        if fields.is_empty() {
            anyhow::bail!(
                "at least one field flag required (--name, --description)"
            );
        }
        build_patch_single("testPlanId", json!(id), fields)
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!("/org/{org}/project/{project}/testPlans");
    let response = client.send(Method::PATCH, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Add imports: `use serde_json::json;` and extend the `super::` import line with `build_patch_single`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit test_plans::`
Expected: all pass.

Run: `cargo test --test unit`
Expected: no regressions.

```bash
git add src/cli/test_plans.rs src/cli/mod.rs src/handlers/test_plans.rs tests/unit/test_plans.rs
git commit -m "feat(test-plans): add flag-mode patch with --id/--name/--description"
```

---

## Task 6: `requirements patch` flag mode

**Files:**
- Modify: `src/cli/requirements.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/requirements.rs`
- Modify: `tests/unit/requirements.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/requirements.rs`:

```rust
use flow_cli::cli::requirements::RequirementPatchArgs;

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_requirements(
        RequirementCommands::Patch(RequirementPatchArgs {
            context: ctx("o", "p"),
            id: Some(2855),
            name: None,
            owner: Some("rio@skl.vc".into()),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PATCH");
    assert_eq!(call.path, "/org/o/project/p/requirements");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "requirementId": 2855, "owner": "rio@skl.vc" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_requirements(
        RequirementCommands::Patch(RequirementPatchArgs {
            context: ctx("o", "p"),
            id: Some(2855),
            name: None,
            owner: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("at least one field flag"));
}
```

(Ensure `use` block at the top of the file imports `RequirementCommands`, `handle_requirements`, etc.)

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit requirements::patch_`
Expected: compile error.

- [ ] **Step 3: Add `RequirementPatchArgs` and wire the CLI**

In `src/cli/requirements.rs`:

- Change the `Patch` variant to `Patch(RequirementPatchArgs)`.
- Add the struct:

```rust
#[derive(Args, Debug)]
pub struct RequirementPatchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub name: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub owner: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export `RequirementPatchArgs` in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

In `src/handlers/requirements.rs`, replace the `RequirementCommands::Patch` arm:

```rust
RequirementCommands::Patch(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(id) = args.id {
        let mut fields = Vec::new();
        if let Some(name) = args.name { fields.push(("name".to_string(), json!(name))); }
        if let Some(owner) = args.owner { fields.push(("owner".to_string(), json!(owner))); }
        if fields.is_empty() {
            anyhow::bail!(
                "at least one field flag required (--name, --owner)"
            );
        }
        build_patch_single("requirementId", json!(id), fields)
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!("/org/{org}/project/{project}/requirements");
    let response = client.send(Method::PATCH, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Add `use serde_json::json;` and extend the `super::` imports with `build_patch_single`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit requirements::`
Expected: all pass.

Run: `cargo test --test unit`
Expected: no regressions.

```bash
git add src/cli/requirements.rs src/cli/mod.rs src/handlers/requirements.rs tests/unit/requirements.rs
git commit -m "feat(requirements): add flag-mode patch with --id/--name/--owner"
```

---

## Task 7: `test-cases set-steps --steps-file`

**Files:**
- Modify: `src/cli/test_cases.rs` (extend `TestCaseItemPayloadArgs` or introduce `TestCaseSetStepsArgs`)
- Modify: `src/handlers/test_cases.rs` (branch on `steps_file` presence)
- Modify: `tests/unit/test_cases.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/test_cases.rs`:

```rust
use std::io::Write;
use flow_cli::cli::test_cases::TestCaseSetStepsArgs;

#[tokio::test]
async fn set_steps_file_mode_wraps_in_server_shape() {
    // write a temp steps file
    let mut f = tempfile::NamedTempFile::new().unwrap();
    writeln!(f, r#"[
      {{"action": "do X", "expected": "Y"}},
      {{"action": "do A", "expected": "B"}}
    ]"#).unwrap();

    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseSetStepsArgs {
            context: ctx("o", "p"),
            id: 7,
            steps_file: Some(f.path().to_path_buf()),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/testCase/7/steps");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{
            "testCaseId": 7,
            "steps": [
                {"action": "do X", "expected": "Y"},
                {"action": "do A", "expected": "B"}
            ]
        }])
    );
}

#[tokio::test]
async fn set_steps_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseSetStepsArgs {
            context: ctx("o", "p"),
            id: 7,
            steps_file: None,
            payload: JsonPayloadArgs {
                json: Some("[]".into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(mock.calls()[0].body.as_ref().unwrap(), &json!([]));
}
```

Add to `tests/unit.rs` if not already present: no changes needed (unit.rs lists modules; we're reusing `test_cases` module).

Add `tempfile = "3"` to `[dev-dependencies]` in `Cargo.toml` (check whether it's already there; if so, skip).

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit test_cases::set_steps_file_mode`
Expected: compile error — `TestCaseSetStepsArgs` missing.

- [ ] **Step 3: Rename `TestCaseItemPayloadArgs` usage for SetSteps to `TestCaseSetStepsArgs`**

In `src/cli/test_cases.rs`:

- Change the variant: `SetSteps(TestCaseSetStepsArgs)`.
- Add the struct:

```rust
#[derive(Args, Debug)]
pub struct TestCaseSetStepsArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long, value_name = "PATH", conflicts_with_all = ["json", "body_file"])]
    pub steps_file: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export `TestCaseSetStepsArgs` in `src/cli/mod.rs` (add to the existing `pub use test_cases::{...}` line).

Other commands that still use `TestCaseItemPayloadArgs` (`LinkJira`, `CreateTestRun`) are unaffected — leave the struct in place for those.

- [ ] **Step 4: Update the handler**

In `src/handlers/test_cases.rs`, replace the `SetSteps` arm:

```rust
TestCaseCommands::SetSteps(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(path) = &args.steps_file {
        let contents = std::fs::read_to_string(path)
            .map_err(|err| anyhow::anyhow!("failed to read {}: {err}", path.display()))?;
        let steps: serde_json::Value = serde_json::from_str(&contents)
            .map_err(|err| anyhow::anyhow!("{} is not valid JSON: {err}", path.display()))?;
        if !steps.is_array() {
            anyhow::bail!("{} must contain a JSON array of steps", path.display());
        }
        json!([{ "testCaseId": args.id, "steps": steps }])
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!("/org/{org}/project/{project}/testCase/{}/steps", args.id);
    let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit test_cases::`
Expected: all pass.

Run: `cargo test --test unit`
Expected: no regressions.

```bash
git add src/cli/test_cases.rs src/cli/mod.rs src/handlers/test_cases.rs tests/unit/test_cases.rs Cargo.toml Cargo.lock
git commit -m "feat(test-cases): add set-steps --steps-file for plain step arrays"
```

---

## Task 8: `requirements link-test-case` flag mode

**Files:**
- Modify: `src/cli/requirements.rs` (extend `RequirementLinkTestCaseArgs`)
- Modify: `src/handlers/requirements.rs`
- Modify: `tests/unit/requirements.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/requirements.rs`:

```rust
use flow_cli::cli::requirements::RequirementLinkTestCaseArgs;

#[tokio::test]
async fn link_test_case_flag_mode_builds_links_wrapper() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_requirements(
        RequirementCommands::LinkTestCase(RequirementLinkTestCaseArgs {
            context: ctx("o", "p"),
            requirement_id: Some(2855),
            test_case_id: Some(326),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/link/requirementTestCase");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!({ "links": [{ "requirementId": 2855, "testCaseId": 326 }] })
    );
}

#[tokio::test]
async fn link_test_case_flag_mode_missing_test_case_id_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_requirements(
        RequirementCommands::LinkTestCase(RequirementLinkTestCaseArgs {
            context: ctx("o", "p"),
            requirement_id: Some(2855),
            test_case_id: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("--test-case-id"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit requirements::link_test_case_flag`
Expected: compile error — new fields not on the struct.

- [ ] **Step 3: Extend `RequirementLinkTestCaseArgs`**

In `src/cli/requirements.rs`, replace the existing struct with:

```rust
#[derive(Args, Debug)]
pub struct RequirementLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub requirement_id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_case_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- [ ] **Step 4: Update the handler**

In `src/handlers/requirements.rs`, replace the `LinkTestCase` arm:

```rust
RequirementCommands::LinkTestCase(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = match (args.requirement_id, args.test_case_id) {
        (Some(rid), Some(tcid)) => build_links_wrapper(vec![
            json!({ "requirementId": rid, "testCaseId": tcid })
        ]),
        (Some(_), None) => anyhow::bail!("--test-case-id is required in flag mode"),
        (None, Some(_)) => anyhow::bail!("--requirement-id is required in flag mode"),
        (None, None) => load_json_payload(&args.payload)?,
    };
    let path = format!("/org/{org}/project/{project}/link/requirementTestCase");
    let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Extend the `super::` imports with `build_links_wrapper`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit requirements::`
Expected: all pass.

```bash
git add src/cli/requirements.rs src/handlers/requirements.rs tests/unit/requirements.rs
git commit -m "feat(requirements): add link-test-case --requirement-id/--test-case-id"
```

---

## Task 9: `test-plans link-test-case` flag mode

**Files:**
- Modify: `src/cli/test_plans.rs` (new `TestPlanLinkTestCaseArgs`)
- Modify: `src/cli/mod.rs` (export new type; the current `TestPlanItemPayloadArgs` keeps serving `LinkTestCaseCrossProject`)
- Modify: `src/handlers/test_plans.rs`
- Modify: `tests/unit/test_plans.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/test_plans.rs`:

```rust
use flow_cli::cli::test_plans::TestPlanLinkTestCaseArgs;

#[tokio::test]
async fn link_test_case_flag_mode_builds_links_wrapper() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_plans(
        TestPlanCommands::LinkTestCase(TestPlanLinkTestCaseArgs {
            context: ctx("o", "p"),
            test_plan_id: Some(203),
            test_case_id: Some(329),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/link/testPlanTestCase");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!({ "links": [{ "testPlanId": 203, "testCaseId": 329 }] })
    );
}

#[tokio::test]
async fn link_test_case_flag_mode_missing_partner_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_plans(
        TestPlanCommands::LinkTestCase(TestPlanLinkTestCaseArgs {
            context: ctx("o", "p"),
            test_plan_id: Some(203),
            test_case_id: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("--test-case-id"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit test_plans::link_test_case_flag`
Expected: compile error.

- [ ] **Step 3: Add `TestPlanLinkTestCaseArgs`**

In `src/cli/test_plans.rs`, locate the `LinkTestCase` variant and change it to:

```rust
LinkTestCase(TestPlanLinkTestCaseArgs),
```

Add the struct at the bottom:

```rust
#[derive(Args, Debug)]
pub struct TestPlanLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_plan_id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_case_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export `TestPlanLinkTestCaseArgs` in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

In `src/handlers/test_plans.rs`, replace the `LinkTestCase` arm:

```rust
TestPlanCommands::LinkTestCase(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = match (args.test_plan_id, args.test_case_id) {
        (Some(pid), Some(tcid)) => build_links_wrapper(vec![
            json!({ "testPlanId": pid, "testCaseId": tcid })
        ]),
        (Some(_), None) => anyhow::bail!("--test-case-id is required in flag mode"),
        (None, Some(_)) => anyhow::bail!("--test-plan-id is required in flag mode"),
        (None, None) => load_json_payload(&args.payload)?,
    };
    let path = format!("/org/{org}/project/{project}/link/testPlanTestCase");
    let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Extend the imports with `build_links_wrapper` and `serde_json::json`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit test_plans::`
Expected: all pass.

```bash
git add src/cli/test_plans.rs src/cli/mod.rs src/handlers/test_plans.rs tests/unit/test_plans.rs
git commit -m "feat(test-plans): add link-test-case --test-plan-id/--test-case-id"
```

---

## Task 10: `systems link-test-plan` flag mode

**Files:**
- Modify: `src/cli/systems.rs` (extend or replace `SystemLinkPayloadArgs` for this variant)
- Modify: `src/handlers/systems.rs`
- Modify: `tests/unit/systems.rs`

**Convention for system-link flag fields:** each of the four `systems link-*` commands gets its own args struct, because each needs a differently-named partner-id flag. Shared struct removed at the end (Task 13).

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/systems.rs`:

```rust
use flow_cli::cli::systems::SystemLinkTestPlanArgs;

#[tokio::test]
async fn link_test_plan_flag_mode_builds_bare_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkTestPlan(SystemLinkTestPlanArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            test_plan_id: Some(203),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/system/sys-uuid/links/testPlans");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testPlanId": 203 }])
    );
}

#[tokio::test]
async fn link_test_plan_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkTestPlan(SystemLinkTestPlanArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            test_plan_id: None,
            payload: JsonPayloadArgs {
                json: Some(r#"[{"testPlanId": 9}]"#.into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(mock.calls()[0].body.as_ref().unwrap(), &json!([{"testPlanId": 9}]));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test unit systems::link_test_plan`
Expected: compile error — `SystemLinkTestPlanArgs` missing.

- [ ] **Step 3: Add `SystemLinkTestPlanArgs`**

In `src/cli/systems.rs`:

- Change the variant: `LinkTestPlan(SystemLinkTestPlanArgs)`.
- Add the struct (copy the shape from `SystemLinkPayloadArgs` and add the flag):

```rust
#[derive(Args, Debug)]
pub struct SystemLinkTestPlanArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_plan_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export `SystemLinkTestPlanArgs` in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

In `src/handlers/systems.rs`, replace the `LinkTestPlan` arm:

```rust
SystemCommands::LinkTestPlan(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(tpid) = args.test_plan_id {
        build_system_link_item("testPlanId", json!(tpid))
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!(
        "/org/{org}/project/{project}/system/{}/links/testPlans",
        args.id
    );
    let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

Extend imports with `build_system_link_item` and `serde_json::json`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit systems::`
Expected: all pass.

```bash
git add src/cli/systems.rs src/cli/mod.rs src/handlers/systems.rs tests/unit/systems.rs
git commit -m "feat(systems): add link-test-plan --test-plan-id"
```

---

## Task 11: `systems link-test-case` flag mode

**Files:**
- Modify: `src/cli/systems.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/systems.rs`
- Modify: `tests/unit/systems.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/systems.rs`:

```rust
use flow_cli::cli::systems::SystemLinkTestCaseArgs;

#[tokio::test]
async fn link_test_case_flag_mode_builds_bare_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkTestCase(SystemLinkTestCaseArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            test_case_id: Some(326),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/system/sys-uuid/links/testCases");
    assert_eq!(call.body.as_ref().unwrap(), &json!([{ "testCaseId": 326 }]));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit systems::link_test_case_flag`
Expected: compile error.

- [ ] **Step 3: Add `SystemLinkTestCaseArgs`**

In `src/cli/systems.rs`:

- Change variant: `LinkTestCase(SystemLinkTestCaseArgs)`.
- Add:

```rust
#[derive(Args, Debug)]
pub struct SystemLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_case_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

Replace the `LinkTestCase` arm in `src/handlers/systems.rs`:

```rust
SystemCommands::LinkTestCase(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(tcid) = args.test_case_id {
        build_system_link_item("testCaseId", json!(tcid))
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!(
        "/org/{org}/project/{project}/system/{}/links/testCases",
        args.id
    );
    let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit systems::`
Expected: all pass.

```bash
git add src/cli/systems.rs src/cli/mod.rs src/handlers/systems.rs tests/unit/systems.rs
git commit -m "feat(systems): add link-test-case --test-case-id"
```

---

## Task 12: `systems link-requirement` flag mode

**Files:**
- Modify: `src/cli/systems.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/systems.rs`
- Modify: `tests/unit/systems.rs`

**Note:** The server schema `AddRequirementToSystemInput` uses the literal key `"id"` (not `"requirementId"`). The CLI flag is still `--requirement-id` for user clarity, but the built payload uses `"id"`.

- [ ] **Step 1: Write failing unit tests**

Append to `tests/unit/systems.rs`:

```rust
use flow_cli::cli::systems::SystemLinkRequirementArgs;

#[tokio::test]
async fn link_requirement_flag_mode_uses_id_key() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkRequirement(SystemLinkRequirementArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            requirement_id: Some(2855),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/system/sys-uuid/links/requirements");
    // AddRequirementToSystemInput uses "id", not "requirementId"
    assert_eq!(call.body.as_ref().unwrap(), &json!([{ "id": 2855 }]));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit systems::link_requirement_flag`
Expected: compile error.

- [ ] **Step 3: Add `SystemLinkRequirementArgs`**

In `src/cli/systems.rs`:

- Change variant: `LinkRequirement(SystemLinkRequirementArgs)`.
- Add:

```rust
#[derive(Args, Debug)]
pub struct SystemLinkRequirementArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub requirement_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

Re-export in `src/cli/mod.rs`.

- [ ] **Step 4: Update the handler**

Replace the `LinkRequirement` arm in `src/handlers/systems.rs`:

```rust
SystemCommands::LinkRequirement(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(rid) = args.requirement_id {
        // AddRequirementToSystemInput takes {"id": <requirementId>}
        build_system_link_item("id", json!(rid))
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!(
        "/org/{org}/project/{project}/system/{}/links/requirements",
        args.id
    );
    let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

- [ ] **Step 5: Run tests and commit**

Run: `cargo test --test unit systems::`
Expected: all pass.

```bash
git add src/cli/systems.rs src/cli/mod.rs src/handlers/systems.rs tests/unit/systems.rs
git commit -m "feat(systems): add link-requirement --requirement-id"
```

---

## Task 13: `systems link-document` flag mode + remove unused `SystemLinkPayloadArgs`

**Files:**
- Modify: `src/cli/systems.rs` (add `SystemLinkDocumentArgs`; remove `SystemLinkPayloadArgs` once no variant uses it)
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/systems.rs`
- Modify: `tests/unit/systems.rs`

- [ ] **Step 1: Write failing unit test**

Append to `tests/unit/systems.rs`:

```rust
use flow_cli::cli::systems::SystemLinkDocumentArgs;

#[tokio::test]
async fn link_document_flag_mode_builds_bare_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkDocument(SystemLinkDocumentArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            document_id: Some("doc-uuid".into()),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/system/sys-uuid/links/documents");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "documentId": "doc-uuid" }])
    );
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test unit systems::link_document_flag`
Expected: compile error.

- [ ] **Step 3: Add `SystemLinkDocumentArgs`, remove dead `SystemLinkPayloadArgs`**

In `src/cli/systems.rs`:

- Change variant: `LinkDocument(SystemLinkDocumentArgs)`.
- Add:

```rust
#[derive(Args, Debug)]
pub struct SystemLinkDocumentArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub document_id: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- Remove `SystemLinkPayloadArgs` (no variant uses it anymore).
- Update `src/cli/mod.rs`: remove `SystemLinkPayloadArgs` from the `pub use systems::{...}` list; add `SystemLinkDocumentArgs`.

- [ ] **Step 4: Update the handler**

Replace the `LinkDocument` arm in `src/handlers/systems.rs`:

```rust
SystemCommands::LinkDocument(args) => {
    let (org, project) = resolve_context(&args.context, config)?;
    let body = if let Some(doc_id) = &args.document_id {
        build_system_link_item("documentId", json!(doc_id))
    } else {
        load_json_payload(&args.payload)?
    };
    let path = format!(
        "/org/{org}/project/{project}/system/{}/links/documents",
        args.id
    );
    let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
    print_output(&response, output)?;
}
```

- [ ] **Step 5: Run the full unit suite and commit**

Run: `cargo test --test unit`
Expected: all tests pass, no regressions.

Run: `cargo build`
Expected: clean build, no warnings about unused `SystemLinkPayloadArgs`.

```bash
git add src/cli/systems.rs src/cli/mod.rs src/handlers/systems.rs tests/unit/systems.rs
git commit -m "feat(systems): add link-document --document-id, remove dead SystemLinkPayloadArgs"
```

---

## Task 14: Update README with flag-mode examples

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Locate the Examples section in the README**

Run: `grep -n "^## Examples\|flow test-cases\|flow requirements" README.md | head`
Note the line around the existing examples.

- [ ] **Step 2: Add a "Flag-mode shortcuts" subsection**

In `README.md`, under the existing Examples section, add:

```markdown
### Flag-mode shortcuts

For single-item mutations, the common fields are available as flags — you don't have to hand-craft the server's JSON payload.

```bash
# Patch
flow test-cases patch --id 326 --owner rio@skl.vc
flow test-plans patch --id 202 --name "Internal Network Performance Test"
flow requirements patch --id 2855 --owner rio@skl.vc

# Set steps from a plain step array (caseStepIds generated server-side)
flow test-cases set-steps --id 326 --steps-file steps.json
#   where steps.json = [{"action": "...", "expected": "..."}, ...]

# Cross-resource links
flow requirements link-test-case --requirement-id 2855 --test-case-id 326
flow test-plans    link-test-case --test-plan-id 203    --test-case-id 329

# System links (the system uses --id; the partner uses --<entity>-id)
flow systems link-test-plan    --id <sys-uuid> --test-plan-id 203
flow systems link-test-case    --id <sys-uuid> --test-case-id 326
flow systems link-requirement  --id <sys-uuid> --requirement-id 2855
flow systems link-document     --id <sys-uuid> --document-id  <doc-uuid>
```

`--json` / `--body-file` continue to work unchanged for batch or custom-field payloads.
```

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "docs(readme): document flag-mode shortcuts for mutations"
```

---

## Task 15: Update the flow-cli skill with the new flag mode

**Files:**
- Modify: `.claude/skills/flow-cli/SKILL.md`

- [ ] **Step 1: Update the "Mutation payload shapes" cheat sheet**

In `.claude/skills/flow-cli/SKILL.md`, locate the section titled `## Mutation payload shapes (cheat sheet)`.

Add a sentence at the top of that section: "Most commands now accept per-field flags that build the payload for you — see below. The raw-JSON forms remain supported for batch and custom-field payloads."

Then update each row of the table to include the flag-mode equivalent alongside the payload shape. Keep the payload-shape column for users who still want the low-level view.

Example row update:

```markdown
| Patch test case | `test-cases patch --id N --owner X` **or** `--json '[{"testCaseId": N, "owner": "X"}]'` | `[{"testCaseId": N, "owner": "..."}]` |
```

Do this for all rows. The "End-to-end: create a test case, own it, step it, link it" recipe below the table should also be updated to use flag mode:

```python
# 2. Set owner (flag mode)
run([FLOW, "test-cases", "patch", "--id", str(tc_id), "--owner", "rio@skl.vc"])

# 3. Set steps from a plain file
# write [{"action": ..., "expected": ...}, ...] to /tmp/steps.json first
run([FLOW, "test-cases", "set-steps", "--id", str(tc_id),
     "--steps-file", "/tmp/steps.json"])

# 4. Link to a requirement
run([FLOW, "requirements", "link-test-case",
     "--requirement-id", "2855", "--test-case-id", str(tc_id)])
```

- [ ] **Step 2: Sync the repo skill to the global skill**

Run: `cp ~/humanoid/flow-cli/.claude/skills/flow-cli/SKILL.md /home/rio/.claude/skills/flow-cli/SKILL.md`

- [ ] **Step 3: Commit**

```bash
git add .claude/skills/flow-cli/SKILL.md
git commit -m "docs(skill): document flag-mode shortcuts for mutations"
```

---

## Final verification

After Task 15, run the full suite and a smoke test:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --release -- test-cases patch --id <a_test_case_you_own> --name "Verify flag-mode" --output json
```

Expected: fmt clean, no clippy warnings, all tests green, and the smoke run against a live test case produces the expected PATCH body (observable via server state or `--output json` response).

---

## Self-review results

**Spec coverage:**
- Summary → covered by all tasks.
- Input modes (flag / body, mutex, error messages) → Tasks 4–13 (each new flag is declared with `conflicts_with_all` against `json`/`body_file`; handler returns the specified error messages).
- Scope table rows:
  - `test-cases patch` → Task 4 ✓
  - `test-cases set-steps` → Task 7 ✓
  - `test-plans patch` → Task 5 ✓
  - `requirements patch` → Task 6 ✓
  - `requirements link-test-case` → Task 8 ✓
  - `test-plans link-test-case` → Task 9 ✓
  - `systems link-test-plan` → Task 10 ✓
  - `systems link-test-case` → Task 11 ✓
  - `systems link-requirement` → Task 12 (`id` key per `AddRequirementToSystemInput`) ✓
  - `systems link-document` → Task 13 ✓
- Payload-builder rules → Tasks 1–3 (builders) + 4–13 (call sites).
- Error handling → covered by per-task negative tests.
- Backward compatibility — `--json` / `--body-file` path — covered by the "json mode still works" tests in Tasks 4, 7, and 10.
- Testing strategy — unit tests per builder and per command; no new integration tests (existing integration suite is read-only and runs unchanged).
- Dependencies: no new crates (`tempfile` added only to `[dev-dependencies]`; plan notes to check whether it's already present).

**Placeholder scan:** no TBDs, no "similar to Task N", every code block is complete.

**Type consistency:** `build_patch_single` / `build_links_wrapper` / `build_system_link_item` are referenced consistently across all tasks with the same signatures defined in Tasks 1–3. Field names on new args structs match what handlers read (`args.id`, `args.name`, `args.owner`, `args.description`, `args.test_case_id`, etc.).
