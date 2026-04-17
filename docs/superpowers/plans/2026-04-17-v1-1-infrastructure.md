# flow-cli v1.0.0 — Plan 1: Infrastructure

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Lay the shared foundation for v1.0.0 — output formatting, testable HTTP layer, per-resource module split, and --output flag wired through main.

**Architecture:** Four concerns in sequence: (1) add `output.rs` and `HttpSend` trait; (2) create test harnesses; (3) split the monolithic `cli.rs` and `handlers.rs` into per-resource modules; (4) thread `OutputFormat` through all handlers and add the `--output` flag to the CLI. No behaviour changes until step 4; steps 1–3 are verified with `cargo build && cargo test`.

**Tech Stack:** Rust edition 2024, clap 4 (derive), reqwest, serde_json, tokio, anyhow.

**Sequence:** Complete this plan before starting plans 2, 3, or 4.

---

### Task 1: Add `src/output.rs`

**Files:**
- Create: `src/output.rs`
- Create: `tests/unit/mod.rs`
- Create: `tests/unit/output.rs`
- Modify: `src/main.rs` (add `mod output;`)

- [ ] **Step 1: Create `tests/unit/output.rs` with failing tests**

```rust
// tests/unit/output.rs
use serde_json::json;

// We import the crate root; these will fail until src/output.rs exists.
use flow_cli::output::{OutputFormat, print_output};

#[test]
fn json_mode_succeeds_for_object() {
    let v = json!({"key": "value", "n": 42});
    assert!(print_output(&v, OutputFormat::Json).is_ok());
}

#[test]
fn json_mode_succeeds_for_array() {
    let v = json!([{"id": 1, "name": "foo"}, {"id": 2, "name": "bar"}]);
    assert!(print_output(&v, OutputFormat::Json).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_empty_array() {
    assert!(print_output(&json!([]), OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_array_of_objects() {
    let v = json!([{"id": 1, "name": "alpha"}, {"id": 2, "name": "beta"}]);
    assert!(print_output(&v, OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_object() {
    let v = json!({"status": 204, "message": "deleted"});
    assert!(print_output(&v, OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_scalar() {
    assert!(print_output(&json!("just a string"), OutputFormat::Table).is_ok());
}
```

- [ ] **Step 2: Create `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
mod output;
```

- [ ] **Step 3: Run tests — expect compile error (output module not yet defined)**

```bash
cargo test --test unit 2>&1 | head -20
```

Expected: error about `flow_cli::output` not found.

- [ ] **Step 4: Create `src/output.rs`**

```rust
// src/output.rs
use anyhow::Result;
use clap::ValueEnum;
use serde_json::Value;

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Json,
    Table,
}

pub fn print_output(value: &Value, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(value)?),
        OutputFormat::Table => print_table(value),
    }
    Ok(())
}

fn print_table(value: &Value) {
    match value {
        Value::Array(items) => print_array_table(items),
        Value::Object(_) => print_object_table(value),
        other => println!("{other}"),
    }
}

fn print_array_table(items: &[Value]) {
    if items.is_empty() {
        println!("(empty)");
        return;
    }
    let headers: Vec<String> = match &items[0] {
        Value::Object(map) => map.keys().cloned().collect(),
        _ => {
            for item in items {
                println!("{}", value_to_cell(item));
            }
            return;
        }
    };
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            headers
                .iter()
                .map(|h| value_to_cell(item.get(h).unwrap_or(&Value::Null)))
                .collect()
        })
        .collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    let header_row: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{header_row}");
    let sep: String = widths.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("  ");
    println!("{sep}");
    for row in &rows {
        let line: String = row
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{:<width$}", cell, width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ");
        println!("{line}");
    }
}

fn print_object_table(value: &Value) {
    if let Value::Object(map) = value {
        let key_width = map.keys().map(|k| k.len()).max().unwrap_or(0);
        for (k, v) in map {
            println!("{:<width$}  {}", k, value_to_cell(v), width = key_width);
        }
    }
}

fn value_to_cell(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
```

- [ ] **Step 5: Add `mod output;` to `src/main.rs`** (add after the existing `mod config;` line)

```rust
mod cli;
mod client;
mod config;
mod handlers;
mod output;
```

- [ ] **Step 6: Run tests — expect all 6 to pass**

```bash
cargo test --test unit
```

Expected: `test output::... ok` × 6.

- [ ] **Step 7: Commit**

```bash
git add src/output.rs src/main.rs tests/unit/mod.rs tests/unit/output.rs
git commit -m "feat: add output module with JSON and table formatting"
```

---

### Task 2: Add `HttpSend` trait to `src/client.rs` and `MockHttpClient` helper

**Files:**
- Modify: `src/client.rs`
- Create: `tests/unit/helpers.rs`
- Modify: `tests/unit/mod.rs`

- [ ] **Step 1: Replace the `send` method on `FlowClient` with a public trait in `src/client.rs`**

Replace the entire contents of `src/client.rs` with:

```rust
// src/client.rs
use anyhow::{Context, Result, bail};
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};

use crate::config::Config;

pub trait HttpSend: Send + Sync {
    async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        with_auth: bool,
    ) -> Result<Value>;
}

#[derive(Clone)]
pub struct FlowClient {
    client: Client,
    base_url: String,
    auth: Auth,
}

#[derive(Clone)]
enum Auth {
    Bearer(String),
    Basic { username: String, password: String },
}

impl FlowClient {
    pub fn from_config(config: &Config) -> Result<Self> {
        let access_token = std::env::var("FLOW_ACCESS_TOKEN")
            .ok()
            .or_else(|| config.access_token.clone());
        let username = std::env::var("FLOW_USERNAME")
            .ok()
            .or_else(|| config.username.clone());
        let password = std::env::var("FLOW_PASSWORD")
            .ok()
            .or_else(|| config.password.clone());

        let auth = if let Some(token) = access_token {
            Auth::Bearer(token)
        } else if let (Some(username), Some(password)) = (username, password) {
            Auth::Basic { username, password }
        } else {
            bail!(
                "no auth configured; use `flow auth exchange`, `flow auth set-bearer`, or `flow auth set-basic`"
            );
        };

        Ok(Self {
            client: Client::builder().build()?,
            base_url: config.effective_base_url(),
            auth,
        })
    }

    pub fn exchange_client(config: &Config) -> Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            base_url: config.effective_base_url(),
            auth: Auth::Bearer(String::new()),
        })
    }
}

impl HttpSend for FlowClient {
    async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        with_auth: bool,
    ) -> Result<Value> {
        let url = format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            normalize_path(path)
        );
        let mut req = self.client.request(method, url).query(query);

        if with_auth {
            req = match &self.auth {
                Auth::Bearer(token) => req.bearer_auth(token),
                Auth::Basic { username, password } => req.basic_auth(username, Some(password)),
            };
        }

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req.send().await.context("request failed")?;
        let status = response.status();

        if status == StatusCode::NO_CONTENT {
            return Ok(json!({ "status": status.as_u16() }));
        }

        let bytes = response
            .bytes()
            .await
            .context("failed to read response body")?;
        if bytes.is_empty() {
            return Ok(json!({ "status": status.as_u16() }));
        }

        let parsed = serde_json::from_slice::<Value>(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));

        if !status.is_success() {
            bail!(
                "request failed with {}: {}",
                status.as_u16(),
                serde_json::to_string_pretty(&parsed)?
            );
        }

        Ok(parsed)
    }
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}
```

- [ ] **Step 2: Run `cargo build` — expect success**

```bash
cargo build 2>&1
```

Expected: `Finished` with no errors. (Handlers still call `client.send(...)` which still works through the trait impl.)

- [ ] **Step 3: Create `tests/unit/helpers.rs`**

```rust
// tests/unit/helpers.rs
use std::sync::{Arc, Mutex};

use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use flow_cli::client::HttpSend;

#[derive(Debug, Clone)]
pub struct MockCall {
    pub method: String,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Value>,
}

pub struct MockHttpClient {
    response: Value,
    calls: Arc<Mutex<Vec<MockCall>>>,
}

impl MockHttpClient {
    pub fn with_response(response: Value) -> Self {
        Self {
            response,
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn calls(&self) -> Vec<MockCall> {
        self.calls.lock().unwrap().clone()
    }
}

impl HttpSend for MockHttpClient {
    async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        _with_auth: bool,
    ) -> Result<Value> {
        self.calls.lock().unwrap().push(MockCall {
            method: method.to_string(),
            path: path.to_string(),
            query: query.to_vec(),
            body,
        });
        Ok(self.response.clone())
    }
}
```

- [ ] **Step 4: Update `tests/unit/mod.rs` to declare the helpers module**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod output;
```

- [ ] **Step 5: Run `cargo test --test unit` — expect all tests still pass**

```bash
cargo test --test unit
```

Expected: all 6 output tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/client.rs tests/unit/helpers.rs tests/unit/mod.rs
git commit -m "feat: add HttpSend trait and MockHttpClient test helper"
```

---

### Task 3: Integration test harness

**Files:**
- Create: `tests/integration/mod.rs`

- [ ] **Step 1: Create `tests/integration/mod.rs`**

```rust
// tests/integration/mod.rs

/// Returns `(token, org, project)` from env vars, or `None` if any are absent.
/// Call this at the top of every integration test and `return` if it returns `None`.
///
/// Example:
/// ```
/// let Some((token, org, project)) = require_credentials() else { return };
/// ```
pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
```

- [ ] **Step 2: Run `cargo test --test integration` — expect it compiles and exits cleanly with 0 tests**

```bash
cargo test --test integration
```

Expected: `running 0 tests` — no failures.

- [ ] **Step 3: Commit**

```bash
git add tests/integration/mod.rs
git commit -m "chore: add integration test harness with credential guard"
```

---

### Task 4: Split `cli.rs` into `src/cli/` module

**Files:**
- Create: `src/cli/mod.rs`
- Create: `src/cli/auth.rs`
- Create: `src/cli/config.rs`
- Create: `src/cli/orgs.rs`
- Create: `src/cli/projects.rs`
- Create: `src/cli/requirements.rs`
- Create: `src/cli/systems.rs`
- Create: `src/cli/test_cases.rs`
- Create: `src/cli/test_plans.rs`
- Create: `src/cli/values.rs`
- Create: `src/cli/util.rs`
- Delete: `src/cli.rs`

This is a pure refactor — no behaviour changes. `cargo build && cargo test` must still pass after this task.

- [ ] **Step 1: Create `src/cli/auth.rs`**

```rust
// src/cli/auth.rs
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Exchange(ExchangeAuthArgs),
    SetBearer(SetBearerArgs),
    SetBasic(SetBasicArgs),
    Status,
    Clear {
        #[arg(long, help = "Also clear the saved refresh token")]
        all: bool,
    },
}

#[derive(Args, Debug)]
pub struct ExchangeAuthArgs {
    #[arg(long, env = "FLOW_REFRESH_TOKEN")]
    pub refresh_token: Option<String>,
    #[arg(long, help = "Persist the refresh token as well as the new access token")]
    pub save_refresh_token: bool,
    #[arg(long, help = "Persist the new access token")]
    pub save: bool,
}

#[derive(Args, Debug)]
pub struct SetBearerArgs {
    #[arg(long, env = "FLOW_ACCESS_TOKEN")]
    pub access_token: String,
    #[arg(long, help = "Persist the access token to the local config")]
    pub save: bool,
}

#[derive(Args, Debug)]
pub struct SetBasicArgs {
    #[arg(long, env = "FLOW_USERNAME")]
    pub username: String,
    #[arg(long, env = "FLOW_PASSWORD")]
    pub password: String,
    #[arg(long, help = "Persist the basic auth credentials to the local config")]
    pub save: bool,
}
```

- [ ] **Step 2: Create `src/cli/config.rs`**

```rust
// src/cli/config.rs
use clap::Subcommand;

use super::ContextArgs;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Show,
    Path,
    SetContext(ContextArgs),
}
```

- [ ] **Step 3: Create `src/cli/orgs.rs`**

```rust
// src/cli/orgs.rs
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum OrgsCommands {
    List,
}
```

- [ ] **Step 4: Create `src/cli/projects.rs`**

```rust
// src/cli/projects.rs
use clap::{Args, Subcommand};

use super::ResourceContextArgs;

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    List(ResourceContextArgs),
    Create(CreateProjectArgs),
}

#[derive(Args, Debug)]
pub struct CreateProjectArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long)]
    pub name: String,
}
```

- [ ] **Step 5: Create `src/cli/requirements.rs`**

```rust
// src/cli/requirements.rs
use clap::Subcommand;

use super::{CreateNamedItemsArgs, ItemArgs, ListArgs, PatchCollectionArgs};

#[derive(Subcommand, Debug)]
pub enum RequirementCommands {
    List(ListArgs),
    Get(ItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(ItemArgs),
}
```

- [ ] **Step 6: Create `src/cli/systems.rs`**

```rust
// src/cli/systems.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ListArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    List(ListArgs),
    Create(CreateSystemArgs),
    Update(UpdateSystemArgs),
    Delete(SystemItemArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SystemItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
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

- [ ] **Step 7: Create `src/cli/test_cases.rs`**

```rust
// src/cli/test_cases.rs
use clap::{Args, Subcommand};

use super::{CreateNamedItemsArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestCaseCommands {
    List(ListArgs),
    Get(TestCaseItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(TestCaseItemArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestCaseItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
```

- [ ] **Step 8: Create `src/cli/test_plans.rs`**

```rust
// src/cli/test_plans.rs
use clap::Subcommand;

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Patch(PatchCollectionArgs),
}
```

- [ ] **Step 9: Create `src/cli/values.rs`**

```rust
// src/cli/values.rs
use clap::{Args, Subcommand};

use super::ResourceContextArgs;

#[derive(Subcommand, Debug)]
pub enum ValueCommands {
    List(ListValuesArgs),
    SetNumber(SetNumberValueArgs),
}

#[derive(Args, Debug)]
pub struct ListValuesArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the numeric values endpoint")]
    pub numeric: bool,
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

- [ ] **Step 10: Create `src/cli/util.rs`**

```rust
// src/cli/util.rs
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    ConvertHtml(ConvertHtmlArgs),
}

#[derive(Args, Debug)]
pub struct ConvertHtmlArgs {
    #[arg(long = "html", required = true)]
    pub html: Vec<String>,
}
```

- [ ] **Step 11: Create `src/cli/mod.rs`**

```rust
// src/cli/mod.rs
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use reqwest::Method;

pub mod auth;
pub mod config;
pub mod orgs;
pub mod projects;
pub mod requirements;
pub mod systems;
pub mod test_cases;
pub mod test_plans;
pub mod values;
pub mod util;

pub use auth::{AuthCommands, ExchangeAuthArgs, SetBasicArgs, SetBearerArgs};
pub use config::ConfigCommands;
pub use orgs::OrgsCommands;
pub use projects::{CreateProjectArgs, ProjectCommands};
pub use requirements::RequirementCommands;
pub use systems::{CreateSystemArgs, SystemCommands, SystemItemArgs, UpdateSystemArgs};
pub use test_cases::{TestCaseCommands, TestCaseItemArgs};
pub use test_plans::TestPlanCommands;
pub use values::{ListValuesArgs, SetNumberValueArgs, ValueCommands};
pub use util::{ConvertHtmlArgs, UtilCommands};

#[derive(Parser, Debug)]
#[command(
    name = "flow",
    about = "CLI for the Flow Engineering REST API",
    version,
    after_help = "Examples:\n  flow auth exchange --refresh-token \"$FLOW_REFRESH_TOKEN\" --save\n  flow config set-context --org my-org --project my-project\n  flow orgs list\n  flow projects list --org my-org\n  flow requirements list --paged --limit 50\n  flow raw GET /orgs\n"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    Orgs {
        #[command(subcommand)]
        command: OrgsCommands,
    },
    Projects {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    Requirements {
        #[command(subcommand)]
        command: RequirementCommands,
    },
    Systems {
        #[command(subcommand)]
        command: SystemCommands,
    },
    TestCases {
        #[command(subcommand)]
        command: TestCaseCommands,
    },
    TestPlans {
        #[command(subcommand)]
        command: TestPlanCommands,
    },
    Values {
        #[command(subcommand)]
        command: ValueCommands,
    },
    Util {
        #[command(subcommand)]
        command: UtilCommands,
    },
    Raw(RawCommand),
}

#[derive(Args, Debug, Clone)]
pub struct ResourceContextArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long, env = "FLOW_PROJECT")]
    pub project: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ContextArgs {
    #[arg(long)]
    pub org: Option<String>,
    #[arg(long)]
    pub project: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the paginated endpoint")]
    pub paged: bool,
    #[arg(long)]
    pub after: Option<String>,
    #[arg(long)]
    pub limit: Option<u32>,
}

#[derive(Args, Debug, Clone)]
pub struct ItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct CreateNamedItemsArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long = "name", required = true)]
    pub names: Vec<String>,
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Args, Debug)]
pub struct PatchCollectionArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug, Default)]
pub struct JsonPayloadArgs {
    #[arg(long, conflicts_with = "body_file")]
    pub json: Option<String>,
    #[arg(long, value_name = "PATH", conflicts_with = "json")]
    pub body_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct RawCommand {
    pub method: HttpMethod,
    pub path: String,
    #[arg(long = "query", value_name = "KEY=VALUE")]
    pub query: Vec<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_method(self) -> Method {
        match self {
            Self::Get => Method::GET,
            Self::Post => Method::POST,
            Self::Put => Method::PUT,
            Self::Patch => Method::PATCH,
            Self::Delete => Method::DELETE,
        }
    }
}
```

- [ ] **Step 12: Delete `src/cli.rs`**

```bash
rm src/cli.rs
```

- [ ] **Step 13: Run `cargo build && cargo test` — expect success**

```bash
cargo build && cargo test
```

Expected: clean compile, all tests pass.

- [ ] **Step 14: Commit**

```bash
git add src/cli/ && git rm src/cli.rs
git commit -m "refactor: split cli.rs into per-resource modules under src/cli/"
```

---

### Task 5: Split `handlers.rs` into `src/handlers/` module

**Files:**
- Create: `src/handlers/mod.rs`
- Create: `src/handlers/auth.rs`
- Create: `src/handlers/config.rs`
- Create: `src/handlers/orgs.rs`
- Create: `src/handlers/projects.rs`
- Create: `src/handlers/requirements.rs`
- Create: `src/handlers/systems.rs`
- Create: `src/handlers/test_cases.rs`
- Create: `src/handlers/test_plans.rs`
- Create: `src/handlers/values.rs`
- Create: `src/handlers/util.rs`
- Delete: `src/handlers.rs`

Pure refactor — no behaviour changes. All shared helper functions stay in `handlers/mod.rs`.

- [ ] **Step 1: Create `src/handlers/auth.rs`**

```rust
// src/handlers/auth.rs
use anyhow::{Result, anyhow};
use reqwest::Method;
use serde_json::{Value, json};
use std::path::Path;

use crate::cli::AuthCommands;
use crate::client::FlowClient;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

pub async fn handle_auth(
    command: AuthCommands,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    match command {
        AuthCommands::Exchange(args) => {
            let refresh_token = args
                .refresh_token
                .or_else(|| config.refresh_token.clone())
                .ok_or_else(|| {
                    anyhow!("no refresh token provided; pass --refresh-token or save one first")
                })?;
            let client = FlowClient::exchange_client(config)?;
            let body = json!({ "refreshToken": refresh_token });
            let response = client
                .send(Method::POST, "/auth/exchange", &[], Some(body), false)
                .await?;

            if args.save || args.save_refresh_token {
                config.access_token = response
                    .get("accessToken")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                if args.save_refresh_token {
                    config.refresh_token = Some(
                        response
                            .get("refreshToken")
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned)
                            .unwrap_or_else(|| refresh_token.clone()),
                    );
                }
                config.save(config_path)?;
            }

            print_output(&response, OutputFormat::Json)?;
        }
        AuthCommands::SetBearer(args) => {
            let response = json!({
                "auth": "bearer",
                "saved": args.save,
                "accessTokenPreview": super::redact(&args.access_token),
            });
            if args.save {
                config.access_token = Some(args.access_token);
                config.username = None;
                config.password = None;
                config.save(config_path)?;
            }
            print_output(&response, OutputFormat::Json)?;
        }
        AuthCommands::SetBasic(args) => {
            let response = json!({
                "auth": "basic",
                "saved": args.save,
                "username": args.username,
            });
            if args.save {
                config.username = response
                    .get("username")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                config.password = Some(args.password);
                config.access_token = None;
                config.save(config_path)?;
            }
            print_output(&response, OutputFormat::Json)?;
        }
        AuthCommands::Status => {
            let auth =
                if std::env::var("FLOW_ACCESS_TOKEN").is_ok() || config.access_token.is_some() {
                    "bearer"
                } else if (std::env::var("FLOW_USERNAME").is_ok()
                    && std::env::var("FLOW_PASSWORD").is_ok())
                    || (config.username.is_some() && config.password.is_some())
                {
                    "basic"
                } else {
                    "none"
                };

            print_output(&json!({
                "auth": auth,
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
                "configPath": config_path.display().to_string(),
                "savedRefreshToken": config.refresh_token.is_some(),
            }), OutputFormat::Json)?;
        }
        AuthCommands::Clear { all } => {
            config.access_token = None;
            config.username = None;
            config.password = None;
            if all {
                config.refresh_token = None;
            }
            config.save(config_path)?;
            print_output(&json!({
                "cleared": "auth",
                "refreshTokenCleared": all,
            }), OutputFormat::Json)?;
        }
    }

    Ok(())
}
```

- [ ] **Step 2: Create `src/handlers/config.rs`**

```rust
// src/handlers/config.rs
use anyhow::Result;
use serde_json::{Value, json};
use std::path::Path;

use crate::cli::ConfigCommands;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

pub fn handle_config(
    command: ConfigCommands,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    match command {
        ConfigCommands::Show => print_output(&json!({
            "configPath": config_path.display().to_string(),
            "baseUrl": config.effective_base_url(),
            "org": config.effective_org(),
            "project": config.effective_project(),
            "hasAccessToken": std::env::var("FLOW_ACCESS_TOKEN").is_ok() || config.access_token.is_some(),
            "hasRefreshToken": config.refresh_token.is_some(),
            "hasBasicAuth": (std::env::var("FLOW_USERNAME").is_ok() && std::env::var("FLOW_PASSWORD").is_ok())
                || (config.username.is_some() && config.password.is_some()),
        }), OutputFormat::Json)?,
        ConfigCommands::Path => {
            println!("{}", config_path.display());
        }
        ConfigCommands::SetContext(args) => {
            if let Some(org) = args.org {
                config.org_alias = Some(org);
            }
            if let Some(project) = args.project {
                config.project_alias = Some(project);
            }
            if let Some(base_url) = args.base_url {
                config.base_url = Some(base_url);
            }
            config.save(config_path)?;
            print_output(&json!({
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
            }), OutputFormat::Json)?;
        }
    }

    Ok(())
}
```

- [ ] **Step 3: Create `src/handlers/orgs.rs`**

```rust
// src/handlers/orgs.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::OrgsCommands;
use crate::client::HttpSend;
use crate::output::{OutputFormat, print_output};

pub async fn handle_orgs<C: HttpSend>(
    command: OrgsCommands,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    match command {
        OrgsCommands::List => {
            let response = client.send(Method::GET, "/orgs", &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 4: Create `src/handlers/projects.rs`**

```rust
// src/handlers/projects.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::ProjectCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::resolve_org;

pub async fn handle_projects<C: HttpSend>(
    command: ProjectCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        ProjectCommands::List(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/projects");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        ProjectCommands::Create(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/projects");
            let response = client
                .send(Method::POST, &path, &[], Some(json!({ "name": args.name })), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 5: Create `src/handlers/requirements.rs`**

```rust
// src/handlers/requirements.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::RequirementCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, named_items_body, patch_collection, resolve_context};

pub async fn handle_requirements<C: HttpSend>(
    command: RequirementCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        RequirementCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/requirements/paged")
            } else {
                format!("/org/{org}/project/{project}/requirements")
            };
            let query = list_query(&args.after, args.limit);
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
            })
            .await?;
        }
        RequirementCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 6: Create `src/handlers/systems.rs`**

```rust
// src/handlers/systems.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::{Value, json};

use crate::cli::SystemCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, load_json_payload, resolve_context};

pub async fn handle_systems<C: HttpSend>(
    command: SystemCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        SystemCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/systems/paged")
            } else {
                format!("/org/{org}/project/{project}/systems")
            };
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
    }
    Ok(())
}
```

- [ ] **Step 7: Create `src/handlers/test_cases.rs`**

```rust
// src/handlers/test_cases.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::TestCaseCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, named_items_body, patch_collection, resolve_context};

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
            })
            .await?;
        }
        TestCaseCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 8: Create `src/handlers/test_plans.rs`**

```rust
// src/handlers/test_plans.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::TestPlanCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

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
        TestPlanCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            })
            .await?;
        }
    }
    Ok(())
}
```

- [ ] **Step 9: Create `src/handlers/values.rs`**

```rust
// src/handlers/values.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::ValueCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::resolve_context;

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
        ValueCommands::SetNumber(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/value/{}/number", args.id);
            let response = client
                .send(Method::PUT, &path, &[], Some(json!({ "value": args.value })), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 10: Create `src/handlers/util.rs`**

```rust
// src/handlers/util.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::{Value, json};

use crate::cli::{RawCommand, UtilCommands};
use crate::client::HttpSend;
use crate::output::{OutputFormat, print_output};

use super::{load_optional_json_payload, parse_query_pair};

pub async fn handle_util<C: HttpSend>(
    command: UtilCommands,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    match command {
        UtilCommands::ConvertHtml(args) => {
            let body = Value::Array(
                args.html
                    .into_iter()
                    .map(|html| json!({ "html": html }))
                    .collect(),
            );
            let response = client
                .send(Method::POST, "/util/convert-html", &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}

pub async fn handle_raw<C: HttpSend>(
    command: RawCommand,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    let query = command
        .query
        .iter()
        .map(|entry| parse_query_pair(entry))
        .collect::<Result<Vec<_>>>()?;
    let body = load_optional_json_payload(&command.payload)?;
    let response = client
        .send(command.method.as_method(), &command.path, &query, body, true)
        .await?;
    print_output(&response, output)?;
    Ok(())
}
```

- [ ] **Step 11: Create `src/handlers/mod.rs`**

```rust
// src/handlers/mod.rs
use anyhow::{Result, anyhow, bail};
use reqwest::Method;
use serde_json::{Value, json};
use std::fs;

use crate::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

pub mod auth;
pub mod config;
pub mod orgs;
pub mod projects;
pub mod requirements;
pub mod systems;
pub mod test_cases;
pub mod test_plans;
pub mod values;
pub mod util;

pub use auth::handle_auth;
pub use config::handle_config;
pub use orgs::handle_orgs;
pub use projects::handle_projects;
pub use requirements::handle_requirements;
pub use systems::handle_systems;
pub use test_cases::handle_test_cases;
pub use test_plans::handle_test_plans;
pub use values::handle_values;
pub use util::{handle_raw, handle_util};

pub(crate) fn resolve_context(
    args: &ResourceContextArgs,
    config: &Config,
) -> Result<(String, String)> {
    let org = resolve_org(&args.org, config)?;
    let project = args
        .project
        .clone()
        .or_else(|| config.effective_project())
        .ok_or_else(|| {
            anyhow!(
                "no project configured; pass --project or set one with `flow config set-context --project ...`"
            )
        })?;
    Ok((org, project))
}

pub(crate) fn resolve_org(org: &Option<String>, config: &Config) -> Result<String> {
    org.clone()
        .or_else(|| config.effective_org())
        .ok_or_else(|| {
            anyhow!(
                "no org configured; pass --org or set one with `flow config set-context --org ...`"
            )
        })
}

pub(crate) fn list_query(after: &Option<String>, limit: Option<u32>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(after) = after {
        query.push(("after".to_string(), after.clone()));
    }
    if let Some(limit) = limit {
        query.push(("limit".to_string(), limit.to_string()));
    }
    query
}

pub(crate) fn load_json_payload(args: &JsonPayloadArgs) -> Result<Value> {
    load_optional_json_payload(args)?
        .ok_or_else(|| anyhow!("request body required; pass --json or --body-file"))
}

pub(crate) fn load_optional_json_payload(args: &JsonPayloadArgs) -> Result<Option<Value>> {
    match (&args.json, &args.body_file) {
        (Some(raw), None) => Ok(Some(parse_json(raw)?)),
        (None, Some(path)) => {
            let contents = fs::read_to_string(path)
                .map_err(|err| anyhow!("failed to read {}: {err}", path.display()))?;
            Ok(Some(parse_json(&contents)?))
        }
        (None, None) => Ok(None),
        _ => bail!("pass only one of --json or --body-file"),
    }
}

pub(crate) fn parse_json(raw: &str) -> Result<Value> {
    serde_json::from_str(raw).map_err(|_| anyhow!("invalid JSON payload"))
}

pub(crate) fn parse_query_pair(input: &str) -> Result<(String, String)> {
    let (key, value) = input
        .split_once('=')
        .ok_or_else(|| anyhow!("invalid query pair `{input}`, expected KEY=VALUE"))?;
    Ok((key.to_string(), value.to_string()))
}

pub(crate) fn redact(value: &str) -> String {
    let suffix = value
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    if value.len() <= 4 {
        "****".to_string()
    } else {
        format!("***{suffix}")
    }
}

pub(crate) fn named_items_body(names: Vec<String>, description: Option<String>) -> Value {
    Value::Array(
        names
            .into_iter()
            .map(|name| {
                let mut item = json!({ "name": name });
                if let Some(description) = &description {
                    item["description"] = Value::String(description.clone());
                }
                item
            })
            .collect(),
    )
}

pub(crate) async fn patch_collection<C, F>(
    client: &C,
    config: &Config,
    args: PatchCollectionArgs,
    output: OutputFormat,
    path: F,
) -> Result<()>
where
    C: HttpSend,
    F: FnOnce(&str, &str) -> String,
{
    let (org, project) = resolve_context(&args.context, config)?;
    let body = load_json_payload(&args.payload)?;
    let response = client
        .send(Method::PATCH, &path(&org, &project), &[], Some(body), true)
        .await?;
    print_output(&response, output)?;
    Ok(())
}
```

- [ ] **Step 12: Delete `src/handlers.rs`**

```bash
rm src/handlers.rs
```

- [ ] **Step 13: Run `cargo build && cargo test` — expect success**

```bash
cargo build && cargo test
```

Expected: clean compile, all tests pass.

- [ ] **Step 14: Commit**

```bash
git add src/handlers/ && git rm src/handlers.rs
git commit -m "refactor: split handlers.rs into per-resource modules under src/handlers/"
```

---

### Task 6: Add `--output` flag to `src/main.rs`

**Files:**
- Modify: `src/main.rs`
- Modify: `src/cli/mod.rs` (add `--output` field to `Cli`)
- Create: `tests/unit/orgs.rs`
- Modify: `tests/unit/mod.rs`

- [ ] **Step 1: Write a failing unit test for `handle_orgs`**

```rust
// tests/unit/orgs.rs
use serde_json::json;

use flow_cli::cli::{OrgsCommands};
use flow_cli::config::Config;
use flow_cli::handlers::handle_orgs;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

#[tokio::test]
async fn orgs_list_calls_get_on_orgs_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_orgs(OrgsCommands::List, &mock, OutputFormat::Json)
        .await
        .unwrap();
    let calls = mock.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].method, "GET");
    assert_eq!(calls[0].path, "/orgs");
}
```

- [ ] **Step 2: Add `mod orgs;` to `tests/unit/mod.rs`**

```rust
// tests/unit/mod.rs
pub mod helpers;
mod orgs;
mod output;
```

- [ ] **Step 3: Run — expect compile error (OutputFormat not yet a parameter of handle_orgs)**

```bash
cargo test --test unit 2>&1 | head -10
```

Expected: error on `handle_orgs` call — wrong number of arguments.

- [ ] **Step 4: Add `--output` flag to `Cli` in `src/cli/mod.rs`**

In `src/cli/mod.rs`, update the `Cli` struct:

```rust
#[derive(Parser, Debug)]
#[command(
    name = "flow",
    about = "CLI for the Flow Engineering REST API",
    version,
    after_help = "Examples:\n  flow auth exchange --refresh-token \"$FLOW_REFRESH_TOKEN\" --save\n  flow config set-context --org my-org --project my-project\n  flow orgs list\n  flow projects list --org my-org\n  flow requirements list --paged --limit 50\n  flow raw GET /orgs\n"
)]
pub struct Cli {
    #[arg(long, global = true, default_value = "json", value_enum)]
    pub output: crate::output::OutputFormat,
    #[command(subcommand)]
    pub command: Commands,
}
```

- [ ] **Step 5: Update `src/main.rs` to thread `output` through all handlers**

Replace the entire contents of `src/main.rs`:

```rust
// src/main.rs
mod cli;
mod client;
mod config;
mod handlers;
mod output;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::client::FlowClient;
use crate::config::{Config, config_path};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = cli.output;
    let config_path = config_path()?;
    let mut config = Config::load(&config_path)?;

    match cli.command {
        Commands::Auth { command } => {
            handlers::handle_auth(command, &mut config, &config_path).await?
        }
        Commands::Config { command } => {
            handlers::handle_config(command, &mut config, &config_path)?
        }
        Commands::Orgs { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_orgs(command, &client, output).await?;
        }
        Commands::Projects { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_projects(command, &client, &config, output).await?;
        }
        Commands::Requirements { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_requirements(command, &client, &config, output).await?;
        }
        Commands::Systems { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_systems(command, &client, &config, output).await?;
        }
        Commands::TestCases { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_cases(command, &client, &config, output).await?;
        }
        Commands::TestPlans { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_plans(command, &client, &config, output).await?;
        }
        Commands::Values { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_values(command, &client, &config, output).await?;
        }
        Commands::Util { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_util(command, &client, output).await?;
        }
        Commands::Raw(command) => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_raw(command, &client, output).await?;
        }
    }

    Ok(())
}
```

- [ ] **Step 6: Run `cargo test --test unit` — expect `orgs_list_calls_get_on_orgs_path` to pass**

```bash
cargo test --test unit
```

Expected: all tests pass including the new orgs test.

- [ ] **Step 7: Run full test suite**

```bash
cargo build && cargo test
```

Expected: clean build, all tests pass.

- [ ] **Step 8: Commit**

```bash
git add src/main.rs src/cli/mod.rs tests/unit/mod.rs tests/unit/orgs.rs
git commit -m "feat: add --output flag (json|table) threaded through all handlers"
```

---

**Plan 1 complete.** Proceed to Plan 2 (existing resource gaps) once `cargo build && cargo test` is green.
