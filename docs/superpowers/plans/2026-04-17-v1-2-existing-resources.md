# flow-cli v1.0.0 — Plan 2: Existing Resource Gaps

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fill in every missing endpoint on the five existing resources — requirements, systems, test-cases, test-plans, and values — including unit tests and integration test skeletons.

**Architecture:** Each task follows the same pattern: add new arg structs and command variants to the resource's `cli/` file, add handler cases to the resource's `handlers/` file, add unit tests (TDD — tests first), then add integration test stubs.

**Tech Stack:** Rust edition 2024, clap 4, reqwest, serde_json, tokio, anyhow.

**Prerequisite:** Plan 1 must be complete (`cargo build && cargo test` green).

---

### Task 7: Requirements — new endpoints

**Files:**
- Modify: `src/cli/requirements.rs`
- Modify: `src/handlers/requirements.rs`
- Create: `tests/unit/requirements.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/requirements.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/requirements.rs`**

```rust
// tests/unit/requirements.rs
use serde_json::json;

use flow_cli::cli::requirements::*;
use flow_cli::cli::{ItemArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs, JsonPayloadArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_requirements;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn list_with_scope_org_calls_organization_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: ListArgs { context: ctx("o", "p"), paged: false, after: None, limit: None },
            scope: Some(RequirementScope::Org),
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/requirements/organization");
}

#[tokio::test]
async fn list_with_scope_without_system_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: ListArgs { context: ctx("o", "p"), paged: false, after: None, limit: None },
            scope: Some(RequirementScope::WithoutSystem),
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/requirements/withoutSystem");
}

#[tokio::test]
async fn filter_calls_post_on_filter_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::Filter(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/requirements/filter");
}

#[tokio::test]
async fn set_stage_calls_put_on_stage_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_requirements(
        RequirementCommands::SetStage(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/requirements/stage");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_requirements(
        RequirementCommands::SetImportId(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/requirements/importid");
}

#[tokio::test]
async fn set_value_calls_put_on_requirements_value_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_requirements(
        RequirementCommands::SetValue(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/requirements/value");
}

#[tokio::test]
async fn list_test_cases_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::ListTestCases(ItemArgs { context: ctx("o", "p"), id: 42 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/requirement/42/testCases");
}

#[tokio::test]
async fn link_jira_calls_post_on_jira_issues_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_requirements(
        RequirementCommands::LinkJira(RequirementJiraArgs {
            context: ctx("o", "p"),
            id: 10,
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/requirement/10/jiraIssues");
}

#[tokio::test]
async fn get_custom_fields_calls_get_on_custom_fields_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::GetCustomFields(ctx("o", "p")),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/requirements/customFields");
}
```

- [ ] **Step 2: Add `mod requirements;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
mod requirements;
```

- [ ] **Step 3: Run — expect compile errors (new types not yet defined)**

```bash
cargo test --test unit 2>&1 | head -20
```

Expected: errors about `ListRequirementsArgs`, `RequirementScope`, etc. not found.

- [ ] **Step 4: Update `src/cli/requirements.rs` with all new commands and arg structs**

```rust
// src/cli/requirements.rs
use clap::{Args, Subcommand, ValueEnum};

use super::{CreateNamedItemsArgs, ItemArgs, JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum RequirementCommands {
    List(ListRequirementsArgs),
    Get(ItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(ItemArgs),
    Filter(PatchCollectionArgs),
    SetStage(PatchCollectionArgs),
    SetImportId(PatchCollectionArgs),
    SetValue(PatchCollectionArgs),
    ListTestCases(ItemArgs),
    ListTestPlans(ItemArgs),
    UploadFile(RequirementFileArgs),
    UploadImage(RequirementImageArgs),
    LinkJira(RequirementJiraArgs),
    UnlinkJira(RequirementUnlinkJiraArgs),
    Link(RequirementLinkArgs),
    Unlink(RequirementUnlinkArgs),
    UnlinkCrossProject(RequirementUnlinkCrossProjectArgs),
    LinkTestCase(RequirementLinkTestCaseArgs),
    LinkTestCaseCrossProject(RequirementLinkTestCaseArgs),
    GetCustomFields(ResourceContextArgs),
    PatchCustomFields(PatchCollectionArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
    AddConfiguration(PatchCollectionArgs),
    RemoveConfiguration(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ListRequirementsArgs {
    #[command(flatten)]
    pub list: ListArgs,
    #[arg(long, value_enum)]
    pub scope: Option<RequirementScope>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RequirementScope {
    Org,
    Project,
    WithoutSystem,
}

#[derive(Args, Debug)]
pub struct RequirementFileArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementImageArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub file_id: String,
}

#[derive(Args, Debug)]
pub struct RequirementJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub jira_issue_id: String,
}

#[derive(Args, Debug)]
pub struct RequirementLinkArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[arg(long)]
    pub linked_requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkCrossProjectArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[arg(long)]
    pub linked_project: String,
    #[arg(long)]
    pub linked_requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct RequirementLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- [ ] **Step 5: Update `src/handlers/requirements.rs` with all new handler arms**

```rust
// src/handlers/requirements.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::requirements::{
    RequirementCommands, RequirementScope,
};
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, load_json_payload, named_items_body, patch_collection, resolve_context};

pub async fn handle_requirements<C: HttpSend>(
    command: RequirementCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        RequirementCommands::List(args) => {
            let (org, project) = resolve_context(&args.list.context, config)?;
            let path = match args.scope {
                None if args.list.paged =>
                    format!("/org/{org}/project/{project}/requirements/paged"),
                None =>
                    format!("/org/{org}/project/{project}/requirements"),
                Some(RequirementScope::Org) =>
                    format!("/org/{org}/project/{project}/requirements/organization"),
                Some(RequirementScope::Project) =>
                    format!("/org/{org}/project/{project}/requirements/project"),
                Some(RequirementScope::WithoutSystem) =>
                    format!("/org/{org}/project/{project}/requirements/withoutSystem"),
            };
            let query = list_query(&args.list.after, args.list.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = named_items_body(args.names, args.description);
            let path = format!("/org/{org}/project/{project}/requirements");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements")
            }).await?;
        }
        RequirementCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Filter(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/filter")
            }).await?;
        }
        RequirementCommands::SetStage(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/stage")
            }).await?;
        }
        RequirementCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/importid")
            }).await?;
        }
        RequirementCommands::SetValue(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/value")
            }).await?;
        }
        RequirementCommands::ListTestCases(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}/testCases", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::ListTestPlans(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}/testPlans", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UploadFile(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}/uploadFile", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UploadImage(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/imageUrl/{}",
                args.id, args.file_id
            );
            let response = client.send(Method::POST, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}/jiraIssues", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UnlinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/jiraIssues/{}",
                args.id, args.jira_issue_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Link(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}",
                args.id, args.link_type
            );
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Unlink(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}/{}",
                args.id, args.link_type, args.linked_requirement_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UnlinkCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}/cross_project/{}/{}",
                args.id, args.link_type, args.linked_project, args.linked_requirement_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/requirementTestCase");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkTestCaseCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/requirementTestCase/crossProject");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::GetCustomFields(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/requirements/customFields");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::PatchCustomFields(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/customFields")
            }).await?;
        }
        RequirementCommands::RenameCustomFieldOption(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/customFields/renameOption")
            }).await?;
        }
        RequirementCommands::AddConfiguration(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/configuration")
            }).await?;
        }
        RequirementCommands::RemoveConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/configuration");
            let response = client.send(Method::DELETE, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Update `src/handlers/mod.rs` to re-export from requirements** — no change needed (already `pub use requirements::handle_requirements`).

- [ ] **Step 7: Run `cargo test --test unit` — expect all tests to pass**

```bash
cargo test --test unit
```

Expected: all unit tests pass including the 9 new requirement tests.

- [ ] **Step 8: Create `tests/integration/requirements.rs`**

```rust
// tests/integration/requirements.rs
use flow_cli::cli::requirements::{ListRequirementsArgs, RequirementCommands};
use flow_cli::cli::{CreateNamedItemsArgs, ItemArgs, JsonPayloadArgs, ListArgs, ResourceContextArgs};
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_requirements;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn requirements_list_returns_array() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: ListArgs { context: ctx(&org, &project), paged: true, after: None, limit: Some(5) },
            scope: None,
        }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}

#[tokio::test]
async fn requirements_create_and_delete_roundtrip() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();

    // Create
    let response = {
        use flow_cli::handlers::requirements::*;
        let mut result = serde_json::Value::Null;
        // We call handle_requirements but need the return value — use client directly for setup
        let body = serde_json::json!([{"name": "Integration test requirement — delete me"}]);
        client.send(reqwest::Method::POST, &format!("/org/{org}/project/{project}/requirements"), &[], Some(body), true).await.unwrap()
    };
    let id = response[0]["id"].as_i64().expect("created requirement has id");

    // Delete
    handle_requirements(
        RequirementCommands::Delete(ItemArgs { context: ctx(&org, &project), id }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 9: Add `mod requirements;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod requirements;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 10: Run full test suite**

```bash
cargo test
```

Expected: all unit tests pass. Integration tests skip if env vars absent.

- [ ] **Step 11: Commit**

```bash
git add src/cli/requirements.rs src/handlers/requirements.rs \
        tests/unit/requirements.rs tests/unit/mod.rs \
        tests/integration/requirements.rs tests/integration/mod.rs
git commit -m "feat: add all missing requirements endpoints"
```

---

### Task 8: Systems — new endpoints

**Files:**
- Modify: `src/cli/systems.rs`
- Modify: `src/handlers/systems.rs`
- Create: `tests/unit/systems.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/systems.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/systems.rs`**

```rust
// tests/unit/systems.rs
use serde_json::json;

use flow_cli::cli::systems::*;
use flow_cli::cli::{JsonPayloadArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_systems;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn bulk_update_calls_put_on_systems_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_systems(
        SystemCommands::BulkUpdate(flow_cli::cli::PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("[]".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/systems");
}

#[tokio::test]
async fn list_documents_calls_get_on_links_documents_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_systems(
        SystemCommands::ListDocuments(SystemItemArgs { context: ctx("o", "p"), id: "sys-1".into() }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/system/sys-1/links/documents");
}

#[tokio::test]
async fn link_requirement_calls_post_on_links_requirements_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_systems(
        SystemCommands::LinkRequirement(SystemLinkPayloadArgs {
            context: ctx("o", "p"),
            id: "sys-1".into(),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/system/sys-1/links/requirements");
}

#[tokio::test]
async fn unlink_test_case_calls_delete_on_correct_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_systems(
        SystemCommands::UnlinkTestCase(SystemUnlinkTestCaseArgs {
            context: ctx("o", "p"),
            id: "sys-1".into(),
            test_case_id: 99,
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/system/sys-1/links/testCase/99");
}
```

- [ ] **Step 2: Add `mod systems;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
mod requirements;
mod systems;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Update `src/cli/systems.rs`**

```rust
// src/cli/systems.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    List(ListArgs),
    Create(CreateSystemArgs),
    Update(UpdateSystemArgs),
    Delete(SystemItemArgs),
    BulkUpdate(PatchCollectionArgs),
    ListDocuments(SystemItemArgs),
    LinkDocument(SystemLinkPayloadArgs),
    ListRequirements(SystemItemArgs),
    LinkRequirement(SystemLinkPayloadArgs),
    UnlinkRequirement(SystemUnlinkRequirementArgs),
    ListTestCases(SystemItemArgs),
    LinkTestCase(SystemLinkPayloadArgs),
    UnlinkTestCase(SystemUnlinkTestCaseArgs),
    ListTestPlans(SystemItemArgs),
    LinkTestPlan(SystemLinkPayloadArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SystemItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct SystemLinkPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct SystemUnlinkRequirementArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct SystemUnlinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub test_case_id: i64,
}

#[derive(Args, Debug)]
pub struct CreateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long, help = "User email, name, or id")]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long, help = "Must match ^[A-Z0-9_-]+$")]
    pub prefix: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long)]
    pub prefix: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- [ ] **Step 5: Update `src/handlers/systems.rs`**

```rust
// src/handlers/systems.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::{Value, json};

use crate::cli::systems::SystemCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, load_json_payload, patch_collection, resolve_context};

pub async fn handle_systems<C: HttpSend>(
    command: SystemCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        SystemCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            // Always use /systems/paged — GET /systems is deprecated in the API.
            // The --paged flag is accepted but ignored for systems.
            let path = format!("/org/{org}/project/{project}/systems/paged");
            let query = list_query(&args.after, args.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let mut body = json!({ "name": args.name });
            if let Some(d) = args.description { body["description"] = Value::String(d); }
            if let Some(o) = args.owner { body["owner"] = Value::String(o); }
            if let Some(p) = args.parent_id { body["parentId"] = Value::String(p); }
            if let Some(p) = args.prefix { body["prefix"] = Value::String(p); }
            let path = format!("/org/{org}/project/{project}/system");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::Update(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = if args.payload.json.is_some() || args.payload.body_file.is_some() {
                load_json_payload(&args.payload)?
            } else {
                let mut body = json!({});
                if let Some(n) = args.name { body["name"] = Value::String(n); }
                if let Some(d) = args.description { body["description"] = Value::String(d); }
                if let Some(o) = args.owner { body["owner"] = Value::String(o); }
                if let Some(p) = args.parent_id { body["parentId"] = Value::String(p); }
                if let Some(p) = args.prefix { body["prefix"] = Value::String(p); }
                body
            };
            let path = format!("/org/{org}/project/{project}/system/{}", args.id);
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::BulkUpdate(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/systems")
            }).await?;
        }
        SystemCommands::ListDocuments(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/documents", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::LinkDocument(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/documents", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::ListRequirements(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/requirements", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::LinkRequirement(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/requirements", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::UnlinkRequirement(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/system/{}/links/requirement/{}",
                args.id, args.requirement_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::ListTestCases(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/testCases", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::LinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/testCases", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::UnlinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/system/{}/links/testCase/{}",
                args.id, args.test_case_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::ListTestPlans(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/testPlans", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::LinkTestPlan(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/system/{}/links/testPlans", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        SystemCommands::RenameCustomFieldOption(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/systems/customFields/renameOption")
            }).await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Create `tests/integration/systems.rs`**

```rust
// tests/integration/systems.rs
use flow_cli::cli::systems::{CreateSystemArgs, SystemCommands, SystemItemArgs};
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_systems;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn systems_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    use flow_cli::cli::ListArgs;
    handle_systems(
        SystemCommands::List(ListArgs {
            context: ctx(&org, &project),
            paged: true,
            after: None,
            limit: Some(5),
        }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 7: Add `mod systems;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod requirements;
mod systems;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 8: Run `cargo test --test unit` — expect all tests pass**

```bash
cargo test --test unit
```

- [ ] **Step 9: Commit**

```bash
git add src/cli/systems.rs src/handlers/systems.rs \
        tests/unit/systems.rs tests/unit/mod.rs \
        tests/integration/systems.rs tests/integration/mod.rs
git commit -m "feat: add all missing systems endpoints"
```

---

### Task 9: Test Cases — new endpoints

**Files:**
- Modify: `src/cli/test_cases.rs`
- Modify: `src/handlers/test_cases.rs`
- Create: `tests/unit/test_cases.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/test_cases.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/test_cases.rs`**

```rust
// tests/unit/test_cases.rs
use serde_json::json;

use flow_cli::cli::test_cases::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_cases;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn set_steps_calls_put_on_steps_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseItemPayloadArgs {
            context: ctx("o", "p"),
            id: 7,
            payload: JsonPayloadArgs { json: Some("[]".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/testCase/7/steps");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_test_cases(
        TestCaseCommands::SetImportId(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/testCases/importid");
}

#[tokio::test]
async fn create_test_run_calls_post_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_cases(
        TestCaseCommands::CreateTestRun(TestCaseItemPayloadArgs {
            context: ctx("o", "p"),
            id: 5,
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/testCase/5/testRun");
}

#[tokio::test]
async fn get_custom_fields_calls_get() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_test_cases(
        TestCaseCommands::GetCustomFields(ctx("o", "p")),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCases/customFields");
}

#[tokio::test]
async fn list_requirements_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_test_cases(
        TestCaseCommands::ListRequirements(flow_cli::cli::test_cases::TestCaseItemArgs {
            context: ctx("o", "p"),
            id: 3,
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/testCase/3/links/requirements");
}
```

- [ ] **Step 2: Add `mod test_cases;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Update `src/cli/test_cases.rs`**

```rust
// src/cli/test_cases.rs
use clap::{Args, Subcommand};

use super::{CreateNamedItemsArgs, JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestCaseCommands {
    List(ListArgs),
    Get(TestCaseItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(TestCaseItemArgs),
    SetSteps(TestCaseItemPayloadArgs),
    SetImportId(PatchCollectionArgs),
    UploadFile(TestCaseUploadFileArgs),
    LinkJira(TestCaseItemPayloadArgs),
    UnlinkJira(TestCaseUnlinkJiraArgs),
    ListRequirements(TestCaseItemArgs),
    GetCustomFields(ResourceContextArgs),
    PatchCustomFields(PatchCollectionArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
    AddConfiguration(PatchCollectionArgs),
    RemoveConfiguration(PatchCollectionArgs),
    SetStages(PatchCollectionArgs),
    CreateTestRun(TestCaseItemPayloadArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestCaseItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestCaseItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct TestCaseUploadFileArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub file_id: String,
}

#[derive(Args, Debug)]
pub struct TestCaseUnlinkJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub jira_issue_id: String,
}
```

- [ ] **Step 5: Update `src/handlers/test_cases.rs`**

```rust
// src/handlers/test_cases.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_cases::TestCaseCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, load_json_payload, named_items_body, patch_collection, resolve_context};

pub async fn handle_test_cases<C: HttpSend>(
    command: TestCaseCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestCaseCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/testCases/paged")
            } else {
                format!("/org/{org}/project/{project}/testCases")
            };
            let query = list_query(&args.after, args.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = named_items_body(args.names, args.description);
            let path = format!("/org/{org}/project/{project}/testCases");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases")
            }).await?;
        }
        TestCaseCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetSteps(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/steps", args.id);
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/importid")
            }).await?;
        }
        TestCaseCommands::UploadFile(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCase/{}/file/{}",
                args.id, args.file_id
            );
            let response = client.send(Method::POST, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::LinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/jiraIssues", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::UnlinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCase/{}/jiraIssues/{}",
                args.id, args.jira_issue_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::ListRequirements(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/links/requirements", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::GetCustomFields(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/testCases/customFields");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::PatchCustomFields(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/customFields")
            }).await?;
        }
        TestCaseCommands::RenameCustomFieldOption(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/customFields/renameOption")
            }).await?;
        }
        TestCaseCommands::AddConfiguration(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/configuration")
            }).await?;
        }
        TestCaseCommands::RemoveConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCases/configuration");
            let response = client.send(Method::DELETE, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetStages(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCaseStages")
            }).await?;
        }
        TestCaseCommands::CreateTestRun(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/testRun", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Create `tests/integration/test_cases.rs`**

```rust
// tests/integration/test_cases.rs
use flow_cli::cli::test_cases::{TestCaseCommands, TestCaseItemArgs};
use flow_cli::cli::{ListArgs, ResourceContextArgs};
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_cases;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn test_cases_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_test_cases(
        TestCaseCommands::List(ListArgs {
            context: ctx(&org, &project),
            paged: true,
            after: None,
            limit: Some(5),
        }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 7: Add `mod test_cases;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod requirements;
mod systems;
mod test_cases;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 8: Run `cargo test --test unit` — expect all tests pass**

```bash
cargo test --test unit
```

- [ ] **Step 9: Commit**

```bash
git add src/cli/test_cases.rs src/handlers/test_cases.rs \
        tests/unit/test_cases.rs tests/unit/mod.rs \
        tests/integration/test_cases.rs tests/integration/mod.rs
git commit -m "feat: add all missing test-cases endpoints"
```

---

### Task 10: Test Plans — add missing CRUD and new endpoints

**Files:**
- Modify: `src/cli/test_plans.rs`
- Modify: `src/handlers/test_plans.rs`
- Modify: `src/cli/mod.rs` (add TestPlans to Commands)
- Create: `tests/unit/test_plans.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/test_plans.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/test_plans.rs`**

```rust
// tests/unit/test_plans.rs
use serde_json::json;

use flow_cli::cli::test_plans::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_plans;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn create_calls_post_on_test_plans_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_plans(
        TestPlanCommands::Create(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/testPlans");
}

#[tokio::test]
async fn get_calls_get_on_test_plan_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 5}));
    handle_test_plans(
        TestPlanCommands::Get(TestPlanItemArgs { context: ctx("o", "p"), id: 5 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testPlan/5");
}

#[tokio::test]
async fn delete_calls_delete_on_test_plan_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_plans(
        TestPlanCommands::Delete(TestPlanItemArgs { context: ctx("o", "p"), id: 3 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testPlan/3");
}

#[tokio::test]
async fn create_cycle_calls_post_on_test_cycle_path() {
    let mock = MockHttpClient::with_response(json!({"id": 10}));
    handle_test_plans(
        TestPlanCommands::CreateCycle(TestPlanItemPayloadArgs {
            context: ctx("o", "p"),
            id: 2,
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/testPlan/2/testCycle");
}

#[tokio::test]
async fn link_test_case_calls_put_on_link_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_test_plans(
        TestPlanCommands::LinkTestCase(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/link/testPlanTestCase");
}
```

- [ ] **Step 2: Add `mod test_plans;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
mod test_plans;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Update `src/cli/test_plans.rs`**

```rust
// src/cli/test_plans.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
    Get(TestPlanItemArgs),
    Patch(PatchCollectionArgs),
    Delete(TestPlanItemArgs),
    CreateCycle(TestPlanItemPayloadArgs),
    SetImportId(PatchCollectionArgs),
    LinkTestCase(PatchCollectionArgs),
    LinkTestCaseCrossProject(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestPlanItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestPlanItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- [ ] **Step 5: Update `src/handlers/test_plans.rs`**

```rust
// src/handlers/test_plans.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_plans::TestPlanCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, patch_collection, resolve_context};

pub async fn handle_test_plans<C: HttpSend>(
    command: TestPlanCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestPlanCommands::List(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/testPlans");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Create(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            }).await?;
        }
        TestPlanCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            }).await?;
        }
        TestPlanCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::CreateCycle(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}/testCycle", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans/importid")
            }).await?;
        }
        TestPlanCommands::LinkTestCase(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/link/testPlanTestCase")
            }).await?;
        }
        TestPlanCommands::LinkTestCaseCrossProject(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/link/testPlanTestCase/crossProject")
            }).await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Create `tests/integration/test_plans.rs`**

```rust
// tests/integration/test_plans.rs
use flow_cli::cli::test_plans::TestPlanCommands;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_plans;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn test_plans_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_test_plans(
        TestPlanCommands::List(ctx(&org, &project)),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 7: Add `mod test_plans;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod requirements;
mod systems;
mod test_cases;
mod test_plans;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 8: Run `cargo test --test unit` — expect all tests pass**

```bash
cargo test --test unit
```

- [ ] **Step 9: Commit**

```bash
git add src/cli/test_plans.rs src/handlers/test_plans.rs \
        tests/unit/test_plans.rs tests/unit/mod.rs \
        tests/integration/test_plans.rs tests/integration/mod.rs
git commit -m "feat: add missing test-plans CRUD and new endpoints"
```

---

### Task 11: Values — add `get` and `set-import-id`

**Files:**
- Modify: `src/cli/values.rs`
- Modify: `src/handlers/values.rs`
- Create: `tests/unit/values.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/values.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/values.rs`**

```rust
// tests/unit/values.rs
use serde_json::json;

use flow_cli::cli::values::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_values;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_value_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_values(
        ValueCommands::Get(ValueItemArgs { context: ctx("o", "p"), id: 99 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/value/99");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_values(
        ValueCommands::SetImportId(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/values/importid");
}
```

- [ ] **Step 2: Add `mod values;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
mod test_plans;
mod values;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Update `src/cli/values.rs`**

```rust
// src/cli/values.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum ValueCommands {
    List(ListValuesArgs),
    Get(ValueItemArgs),
    SetNumber(SetNumberValueArgs),
    SetImportId(PatchCollectionArgs),
}

#[derive(Args, Debug)]
pub struct ListValuesArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the numeric values endpoint")]
    pub numeric: bool,
}

#[derive(Args, Debug)]
pub struct ValueItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct SetNumberValueArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub value: f64,
}
```

- [ ] **Step 5: Update `src/handlers/values.rs`**

```rust
// src/handlers/values.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::values::ValueCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

pub async fn handle_values<C: HttpSend>(
    command: ValueCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        ValueCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let suffix = if args.numeric { "values/number" } else { "values" };
            let path = format!("/org/{org}/project/{project}/{suffix}");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        ValueCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/value/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        ValueCommands::SetNumber(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/value/{}/number", args.id);
            let response = client
                .send(Method::PUT, &path, &[], Some(json!({ "value": args.value })), true)
                .await?;
            print_output(&response, output)?;
        }
        ValueCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/values/importid")
            }).await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Create `tests/integration/values.rs`**

```rust
// tests/integration/values.rs
use flow_cli::cli::values::{ListValuesArgs, ValueCommands};
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_values;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn values_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_values(
        ValueCommands::List(ListValuesArgs { context: ctx(&org, &project), numeric: false }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 7: Add `mod values;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod requirements;
mod systems;
mod test_cases;
mod test_plans;
mod values;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 8: Run full test suite**

```bash
cargo test
```

Expected: all unit tests pass. Integration tests skip without credentials.

- [ ] **Step 9: Commit**

```bash
git add src/cli/values.rs src/handlers/values.rs \
        tests/unit/values.rs tests/unit/mod.rs \
        tests/integration/values.rs tests/integration/mod.rs
git commit -m "feat: add values get and set-import-id endpoints"
```

---

**Plan 2 complete.** Proceed to Plan 3 (new resources) once `cargo test` is green.
