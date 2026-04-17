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

pub(crate) fn resolve_context(args: &ResourceContextArgs, config: &Config) -> Result<(String, String)> {
    let org = resolve_org(&args.org, config)?;
    let project = args
        .project
        .clone()
        .or_else(|| config.effective_project())
        .ok_or_else(|| anyhow!("no project configured; pass --project or set one with `flow config set-context --project ...`"))?;
    Ok((org, project))
}

pub(crate) fn resolve_org(org: &Option<String>, config: &Config) -> Result<String> {
    org.clone()
        .or_else(|| config.effective_org())
        .ok_or_else(|| anyhow!("no org configured; pass --org or set one with `flow config set-context --org ...`"))
}

pub(crate) fn list_query(after: &Option<String>, limit: Option<u32>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(after) = after { query.push(("after".to_string(), after.clone())); }
    if let Some(limit) = limit { query.push(("limit".to_string(), limit.to_string())); }
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
    let suffix = value.chars().rev().take(4).collect::<String>().chars().rev().collect::<String>();
    if value.len() <= 4 { "****".to_string() } else { format!("***{suffix}") }
}

pub(crate) fn named_items_body(names: Vec<String>, description: Option<String>) -> Value {
    Value::Array(
        names.into_iter().map(|name| {
            let mut item = json!({ "name": name });
            if let Some(description) = &description { item["description"] = Value::String(description.clone()); }
            item
        }).collect(),
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
