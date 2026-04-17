# flow-cli v1.0.0 — Plan 3: New Resources

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement six brand-new top-level commands: documents, interfaces, members, configurations, test-cycles, and test-runs.

**Architecture:** Each task follows the same four-file pattern: `src/cli/{resource}.rs` (clap structs) → `src/handlers/{resource}.rs` (async handler) → register in `src/cli/mod.rs`, `src/handlers/mod.rs`, and `src/main.rs` → unit + integration tests. After Task 12 the pattern is established; subsequent tasks repeat it exactly.

**Tech Stack:** Rust edition 2024, clap 4, reqwest, serde_json, tokio, anyhow.

**Prerequisite:** Plans 1 and 2 must be complete (`cargo test` green).

---

### Task 12: Documents

**Files:**
- Create: `src/cli/documents.rs`
- Create: `src/handlers/documents.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/mod.rs`
- Modify: `src/main.rs`
- Create: `tests/unit/documents.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/documents.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/documents.rs`**

```rust
// tests/unit/documents.rs
use serde_json::json;

use flow_cli::cli::documents::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_documents;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn list_calls_get_on_documents_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_documents(
        DocumentCommands::List(ctx("o", "p")),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/documents/paged");
}

#[tokio::test]
async fn get_calls_get_on_document_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_documents(
        DocumentCommands::Get(DocumentItemArgs { context: ctx("o", "p"), id: 7 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/document/7");
}

#[tokio::test]
async fn create_calls_post_on_documents_path() {
    let mock = MockHttpClient::with_response(json!({"id": 2}));
    handle_documents(
        DocumentCommands::Create(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/documents");
}

#[tokio::test]
async fn delete_calls_delete_on_document_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_documents(
        DocumentCommands::Delete(DocumentItemArgs { context: ctx("o", "p"), id: 3 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/document/3");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_documents(
        DocumentCommands::SetImportId(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/documents/importid");
}
```

- [ ] **Step 2: Add `mod documents;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod documents;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
mod test_plans;
mod values;
```

- [ ] **Step 3: Run — expect compile errors (handle_documents and DocumentCommands not found)**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Create `src/cli/documents.rs`**

```rust
// src/cli/documents.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum DocumentCommands {
    List(ResourceContextArgs),
    Get(DocumentItemArgs),
    Create(PatchCollectionArgs),
    Patch(PatchCollectionArgs),
    Delete(DocumentItemArgs),
    SetImportId(PatchCollectionArgs),
}

#[derive(Args, Debug)]
pub struct DocumentItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
```

- [ ] **Step 5: Create `src/handlers/documents.rs`**

```rust
// src/handlers/documents.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::documents::DocumentCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

pub async fn handle_documents<C: HttpSend>(
    command: DocumentCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        DocumentCommands::List(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/documents/paged");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        DocumentCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/document/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        DocumentCommands::Create(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/documents")
            }).await?;
        }
        DocumentCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/documents")
            }).await?;
        }
        DocumentCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/document/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        DocumentCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/documents/importid")
            }).await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register documents in `src/cli/mod.rs`**

Add at the top of the `mod` and `use` section:
```rust
pub mod documents;
pub use documents::{DocumentCommands, DocumentItemArgs};
```

Add to the `Commands` enum:
```rust
Documents {
    #[command(subcommand)]
    command: DocumentCommands,
},
```

- [ ] **Step 7: Register documents in `src/handlers/mod.rs`**

Add:
```rust
pub mod documents;
pub use documents::handle_documents;
```

- [ ] **Step 8: Add routing in `src/main.rs`**

Add to the `match cli.command` block:
```rust
Commands::Documents { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_documents(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Run `cargo test --test unit` — expect all tests pass**

```bash
cargo test --test unit
```

- [ ] **Step 10: Create `tests/integration/documents.rs`**

```rust
// tests/integration/documents.rs
use flow_cli::cli::documents::DocumentCommands;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_documents;
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
async fn documents_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_documents(
        DocumentCommands::List(ctx(&org, &project)),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 11: Add `mod documents;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod documents;
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

- [ ] **Step 12: Run `cargo build && cargo test`**

```bash
cargo build && cargo test
```

Expected: all tests pass.

- [ ] **Step 13: Commit**

```bash
git add src/cli/documents.rs src/handlers/documents.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/documents.rs tests/unit/mod.rs \
        tests/integration/documents.rs tests/integration/mod.rs
git commit -m "feat: add documents command"
```

---

### Task 13: Interfaces

**Files:**
- Create: `src/cli/interfaces.rs`
- Create: `src/handlers/interfaces.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/handlers/mod.rs`
- Modify: `src/main.rs`
- Create: `tests/unit/interfaces.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/interfaces.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/interfaces.rs`**

```rust
// tests/unit/interfaces.rs
use serde_json::json;

use flow_cli::cli::interfaces::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_interfaces;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn list_calls_get_on_interfaces_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_interfaces(
        InterfaceCommands::List(ctx("o", "p")),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/interfaces/paged");
}

#[tokio::test]
async fn create_calls_post_on_interfaces_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_interfaces(
        InterfaceCommands::Create(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/interfaces");
}

#[tokio::test]
async fn delete_calls_delete_on_interface_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_interfaces(
        InterfaceCommands::Delete(InterfaceItemArgs { context: ctx("o", "p"), id: 5 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/interface/5");
}
```

- [ ] **Step 2: Add `mod interfaces;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod documents;
mod interfaces;
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

- [ ] **Step 4: Create `src/cli/interfaces.rs`**

```rust
// src/cli/interfaces.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum InterfaceCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
    Patch(PatchCollectionArgs),
    Delete(InterfaceItemArgs),
}

#[derive(Args, Debug)]
pub struct InterfaceItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
```

- [ ] **Step 5: Create `src/handlers/interfaces.rs`**

```rust
// src/handlers/interfaces.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::interfaces::InterfaceCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

pub async fn handle_interfaces<C: HttpSend>(
    command: InterfaceCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        InterfaceCommands::List(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/interfaces/paged");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        InterfaceCommands::Create(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/interfaces")
            }).await?;
        }
        InterfaceCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/interfaces")
            }).await?;
        }
        InterfaceCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/interface/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register interfaces in `src/cli/mod.rs`** — add `pub mod interfaces;`, `pub use interfaces::{InterfaceCommands, InterfaceItemArgs};`, and `Interfaces { #[command(subcommand)] command: InterfaceCommands }` to Commands.

- [ ] **Step 7: Register interfaces in `src/handlers/mod.rs`** — add `pub mod interfaces;` and `pub use interfaces::handle_interfaces;`.

- [ ] **Step 8: Add routing in `src/main.rs`**

```rust
Commands::Interfaces { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_interfaces(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Create `tests/integration/interfaces.rs`**

```rust
// tests/integration/interfaces.rs
use flow_cli::cli::interfaces::InterfaceCommands;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_interfaces;
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
async fn interfaces_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_interfaces(
        InterfaceCommands::List(ctx(&org, &project)),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 10: Add `mod interfaces;` to `tests/integration/mod.rs`** — add `mod interfaces;` alongside the other mods.

- [ ] **Step 11: Run `cargo test --test unit && cargo build`**

```bash
cargo test --test unit && cargo build
```

Expected: all tests pass, clean build.

- [ ] **Step 12: Commit**

```bash
git add src/cli/interfaces.rs src/handlers/interfaces.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/interfaces.rs tests/unit/mod.rs \
        tests/integration/interfaces.rs tests/integration/mod.rs
git commit -m "feat: add interfaces command"
```

---

### Task 14: Members

**Files:**
- Create: `src/cli/members.rs`
- Create: `src/handlers/members.rs`
- Modify: `src/cli/mod.rs`, `src/handlers/mod.rs`, `src/main.rs`
- Create: `tests/unit/members.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/members.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/members.rs`**

```rust
// tests/unit/members.rs
use serde_json::json;

use flow_cli::cli::members::*;
use flow_cli::cli::{JsonPayloadArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_members;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

#[tokio::test]
async fn list_org_calls_get_on_org_members_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_members(
        MemberCommands::ListOrg(OrgMemberArgs { org: Some("my-org".into()) }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/my-org/members");
}

#[tokio::test]
async fn remove_org_calls_delete_on_org_member_email_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_members(
        MemberCommands::RemoveOrg(OrgRemoveMemberArgs {
            org: Some("my-org".into()),
            email: "user@example.com".into(),
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/my-org/members/user@example.com");
}

#[tokio::test]
async fn list_project_calls_get_on_project_members_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_members(
        MemberCommands::ListProject(ProjectMemberArgs {
            context: ResourceContextArgs { org: Some("o".into()), project: Some("p".into()) },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/members");
}

#[tokio::test]
async fn add_project_calls_post_on_project_members_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_members(
        MemberCommands::AddProject(ProjectAddMemberArgs {
            context: ResourceContextArgs { org: Some("o".into()), project: Some("p".into()) },
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/members");
}
```

- [ ] **Step 2: Add `mod members;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod documents;
mod interfaces;
mod members;
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

- [ ] **Step 4: Create `src/cli/members.rs`**

```rust
// src/cli/members.rs
use clap::{Args, Subcommand};
use std::path::PathBuf;

use super::{JsonPayloadArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum MemberCommands {
    ListOrg(OrgMemberArgs),
    AddOrg(OrgAddMemberArgs),
    RemoveOrg(OrgRemoveMemberArgs),
    ListProject(ProjectMemberArgs),
    AddProject(ProjectAddMemberArgs),
    RemoveProject(ProjectRemoveMemberArgs),
}

#[derive(Args, Debug)]
pub struct OrgMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
}

#[derive(Args, Debug)]
pub struct OrgAddMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct OrgRemoveMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long)]
    pub email: String,
}

#[derive(Args, Debug)]
pub struct ProjectMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
}

#[derive(Args, Debug)]
pub struct ProjectAddMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct ProjectRemoveMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub email: String,
}
```

- [ ] **Step 5: Create `src/handlers/members.rs`**

```rust
// src/handlers/members.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::members::MemberCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, resolve_context, resolve_org};

pub async fn handle_members<C: HttpSend>(
    command: MemberCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        MemberCommands::ListOrg(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/members");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        MemberCommands::AddOrg(args) => {
            let org = resolve_org(&args.org, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/members");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        MemberCommands::RemoveOrg(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/members/{}", args.email);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        MemberCommands::ListProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/members");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        MemberCommands::AddProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/members");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        MemberCommands::RemoveProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/members/{}", args.email);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register members in `src/cli/mod.rs`** — add `pub mod members;`, `pub use members::MemberCommands;`, and `Members { #[command(subcommand)] command: MemberCommands }` to Commands.

- [ ] **Step 7: Register members in `src/handlers/mod.rs`** — add `pub mod members;` and `pub use members::handle_members;`.

- [ ] **Step 8: Add routing in `src/main.rs`**

```rust
Commands::Members { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_members(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Create `tests/integration/members.rs`**

```rust
// tests/integration/members.rs
use flow_cli::cli::members::{MemberCommands, OrgMemberArgs};
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_members;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, _project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c
}

#[tokio::test]
async fn members_list_org_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_members(
        MemberCommands::ListOrg(OrgMemberArgs { org: Some(org) }),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 10: Add `mod members;` to `tests/integration/mod.rs`** — add `mod members;` alongside the other mods.

- [ ] **Step 11: Run `cargo test --test unit && cargo build`**

```bash
cargo test --test unit && cargo build
```

- [ ] **Step 12: Commit**

```bash
git add src/cli/members.rs src/handlers/members.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/members.rs tests/unit/mod.rs \
        tests/integration/members.rs tests/integration/mod.rs
git commit -m "feat: add members command"
```

---

### Task 15: Configurations

**Files:**
- Create: `src/cli/configurations.rs`
- Create: `src/handlers/configurations.rs`
- Modify: `src/cli/mod.rs`, `src/handlers/mod.rs`, `src/main.rs`
- Create: `tests/unit/configurations.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/configurations.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/configurations.rs`**

```rust
// tests/unit/configurations.rs
use serde_json::json;

use flow_cli::cli::configurations::*;
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_configurations;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn list_calls_get_on_configurations_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_configurations(
        ConfigurationCommands::List(ctx("o", "p")),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/configurations");
}

#[tokio::test]
async fn create_calls_post_on_configurations_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_configurations(
        ConfigurationCommands::Create(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/configurations");
}
```

- [ ] **Step 2: Add `mod configurations;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod configurations;
mod documents;
mod interfaces;
mod members;
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

- [ ] **Step 4: Create `src/cli/configurations.rs`**

```rust
// src/cli/configurations.rs
use clap::Subcommand;

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum ConfigurationCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
}
```

- [ ] **Step 5: Create `src/handlers/configurations.rs`**

```rust
// src/handlers/configurations.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::configurations::ConfigurationCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

pub async fn handle_configurations<C: HttpSend>(
    command: ConfigurationCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        ConfigurationCommands::List(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/configurations");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        ConfigurationCommands::Create(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/configurations")
            }).await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register configurations in `src/cli/mod.rs`** — add `pub mod configurations;`, `pub use configurations::ConfigurationCommands;`, and `Configurations { #[command(subcommand)] command: ConfigurationCommands }` to Commands.

- [ ] **Step 7: Register configurations in `src/handlers/mod.rs`** — add `pub mod configurations;` and `pub use configurations::handle_configurations;`.

- [ ] **Step 8: Add routing in `src/main.rs`**

```rust
Commands::Configurations { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_configurations(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Create `tests/integration/configurations.rs`**

```rust
// tests/integration/configurations.rs
use flow_cli::cli::configurations::ConfigurationCommands;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_configurations;
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
async fn configurations_list_succeeds() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_configurations(
        ConfigurationCommands::List(ctx(&org, &project)),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
```

- [ ] **Step 10: Add `mod configurations;` to `tests/integration/mod.rs`** — add `mod configurations;` alongside the other mods.

- [ ] **Step 11: Run `cargo test --test unit && cargo build`**

```bash
cargo test --test unit && cargo build
```

- [ ] **Step 12: Commit**

```bash
git add src/cli/configurations.rs src/handlers/configurations.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/configurations.rs tests/unit/mod.rs \
        tests/integration/configurations.rs tests/integration/mod.rs
git commit -m "feat: add configurations command"
```

---

### Task 16: Test Cycles

**Files:**
- Create: `src/cli/test_cycles.rs`
- Create: `src/handlers/test_cycles.rs`
- Modify: `src/cli/mod.rs`, `src/handlers/mod.rs`, `src/main.rs`
- Create: `tests/unit/test_cycles.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/test_cycles.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/test_cycles.rs`**

```rust
// tests/unit/test_cycles.rs
use serde_json::json;

use flow_cli::cli::test_cycles::*;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_cycles;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_test_cycle_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_cycles(
        TestCycleCommands::Get(TestCycleItemArgs { context: ctx("o", "p"), id: 42 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCycle/42");
}

#[tokio::test]
async fn delete_calls_delete_on_test_cycle_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_cycles(
        TestCycleCommands::Delete(TestCycleItemArgs { context: ctx("o", "p"), id: 7 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testCycle/7");
}
```

- [ ] **Step 2: Add `mod test_cycles;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod configurations;
mod documents;
mod interfaces;
mod members;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
mod test_cycles;
mod test_plans;
mod values;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Create `src/cli/test_cycles.rs`**

```rust
// src/cli/test_cycles.rs
use clap::{Args, Subcommand};

use super::ResourceContextArgs;

#[derive(Subcommand, Debug)]
pub enum TestCycleCommands {
    Get(TestCycleItemArgs),
    Delete(TestCycleItemArgs),
}

#[derive(Args, Debug)]
pub struct TestCycleItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
```

- [ ] **Step 5: Create `src/handlers/test_cycles.rs`**

```rust
// src/handlers/test_cycles.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_cycles::TestCycleCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::resolve_context;

pub async fn handle_test_cycles<C: HttpSend>(
    command: TestCycleCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestCycleCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCycle/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCycleCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCycle/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register test-cycles in `src/cli/mod.rs`** — add `pub mod test_cycles;`, `pub use test_cycles::{TestCycleCommands, TestCycleItemArgs};`, and `TestCycles { #[command(subcommand)] command: TestCycleCommands }` to Commands.

- [ ] **Step 7: Register test-cycles in `src/handlers/mod.rs`** — add `pub mod test_cycles;` and `pub use test_cycles::handle_test_cycles;`.

- [ ] **Step 8: Add routing in `src/main.rs`**

```rust
Commands::TestCycles { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_test_cycles(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Create `tests/integration/test_cycles.rs`**

```rust
// tests/integration/test_cycles.rs
// Test cycles are created via `test-plans create-cycle`.
// Integration tests here verify get/delete assuming an existing cycle ID.
// Skip if FLOW_TEST_CYCLE_ID is not set.

#[tokio::test]
async fn test_cycles_get_skips_without_cycle_id() {
    // Requires FLOW_ACCESS_TOKEN, FLOW_ORG, FLOW_PROJECT, FLOW_TEST_CYCLE_ID
    let Some(_) = std::env::var("FLOW_TEST_CYCLE_ID").ok() else { return };
}
```

- [ ] **Step 10: Add `mod test_cycles;` to `tests/integration/mod.rs`** — add `mod test_cycles;` alongside the other mods.

- [ ] **Step 11: Run `cargo test --test unit && cargo build`**

```bash
cargo test --test unit && cargo build
```

- [ ] **Step 12: Commit**

```bash
git add src/cli/test_cycles.rs src/handlers/test_cycles.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/test_cycles.rs tests/unit/mod.rs \
        tests/integration/test_cycles.rs tests/integration/mod.rs
git commit -m "feat: add test-cycles command"
```

---

### Task 17: Test Runs

**Files:**
- Create: `src/cli/test_runs.rs`
- Create: `src/handlers/test_runs.rs`
- Modify: `src/cli/mod.rs`, `src/handlers/mod.rs`, `src/main.rs`
- Create: `tests/unit/test_runs.rs`
- Modify: `tests/unit/mod.rs`
- Create: `tests/integration/test_runs.rs`
- Modify: `tests/integration/mod.rs`

- [ ] **Step 1: Write failing unit tests in `tests/unit/test_runs.rs`**

```rust
// tests/unit/test_runs.rs
use serde_json::json;

use flow_cli::cli::test_runs::*;
use flow_cli::cli::{JsonPayloadArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_runs;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_runs(
        TestRunCommands::Get(TestRunItemArgs { context: ctx("o", "p"), cycle_id: 10, id: 1 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCycle/10/testRun/1");
}

#[tokio::test]
async fn delete_calls_delete_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_runs(
        TestRunCommands::Delete(TestRunItemArgs { context: ctx("o", "p"), cycle_id: 10, id: 2 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testCycle/10/testRun/2");
}

#[tokio::test]
async fn patch_calls_patch_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_test_runs(
        TestRunCommands::Patch(TestRunItemPayloadArgs {
            context: ctx("o", "p"),
            cycle_id: 5,
            id: 3,
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PATCH");
    assert_eq!(call.path, "/org/o/project/p/testCycle/5/testRun/3");
}

#[tokio::test]
async fn set_steps_calls_put_on_steps_path() {
    let mock = MockHttpClient::with_response(json!({"status": 200}));
    handle_test_runs(
        TestRunCommands::SetSteps(TestRunItemPayloadArgs {
            context: ctx("o", "p"),
            cycle_id: 5,
            id: 3,
            payload: JsonPayloadArgs { json: Some("[]".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/testCycle/5/testRun/3/steps");
}
```

- [ ] **Step 2: Add `mod test_runs;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod configurations;
mod documents;
mod interfaces;
mod members;
mod orgs;
mod output;
mod requirements;
mod systems;
mod test_cases;
mod test_cycles;
mod test_plans;
mod test_runs;
mod values;
```

- [ ] **Step 3: Run — expect compile errors**

```bash
cargo test --test unit 2>&1 | head -20
```

- [ ] **Step 4: Create `src/cli/test_runs.rs`**

```rust
// src/cli/test_runs.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestRunCommands {
    Get(TestRunItemArgs),
    Patch(TestRunItemPayloadArgs),
    Delete(TestRunItemArgs),
    SetSteps(TestRunItemPayloadArgs),
}

#[derive(Args, Debug)]
pub struct TestRunItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub cycle_id: i64,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestRunItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub cycle_id: i64,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
```

- [ ] **Step 5: Create `src/handlers/test_runs.rs`**

```rust
// src/handlers/test_runs.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_runs::TestRunCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, resolve_context};

pub async fn handle_test_runs<C: HttpSend>(
    command: TestRunCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestRunCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestRunCommands::Patch(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client.send(Method::PATCH, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestRunCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestRunCommands::SetSteps(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}/steps",
                args.cycle_id, args.id
            );
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Register test-runs in `src/cli/mod.rs`** — add `pub mod test_runs;`, `pub use test_runs::{TestRunCommands, TestRunItemArgs, TestRunItemPayloadArgs};`, and `TestRuns { #[command(subcommand)] command: TestRunCommands }` to Commands.

- [ ] **Step 7: Register test-runs in `src/handlers/mod.rs`** — add `pub mod test_runs;` and `pub use test_runs::handle_test_runs;`.

- [ ] **Step 8: Add routing in `src/main.rs`**

```rust
Commands::TestRuns { command } => {
    let client = FlowClient::from_config(&config)?;
    handlers::handle_test_runs(command, &client, &config, output).await?;
}
```

- [ ] **Step 9: Create `tests/integration/test_runs.rs`**

```rust
// tests/integration/test_runs.rs
// Test runs require an existing test cycle. Skip unless FLOW_TEST_CYCLE_ID is set.

#[tokio::test]
async fn test_runs_skip_without_cycle_id() {
    let Some(_) = std::env::var("FLOW_TEST_CYCLE_ID").ok() else { return };
}
```

- [ ] **Step 10: Add `mod test_runs;` to `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs
mod configurations;
mod documents;
mod interfaces;
mod members;
mod requirements;
mod systems;
mod test_cases;
mod test_cycles;
mod test_plans;
mod test_runs;
mod values;

pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 11: Run full test suite**

```bash
cargo test
```

Expected: all unit tests pass, integration tests skip without credentials.

- [ ] **Step 12: Commit**

```bash
git add src/cli/test_runs.rs src/handlers/test_runs.rs \
        src/cli/mod.rs src/handlers/mod.rs src/main.rs \
        tests/unit/test_runs.rs tests/unit/mod.rs \
        tests/integration/test_runs.rs tests/integration/mod.rs
git commit -m "feat: add test-runs command"
```

---

**Plan 3 complete.** Proceed to Plan 4 (CI/CD + release) once `cargo test` is green.
