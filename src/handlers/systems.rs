// src/handlers/systems.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::{Value, json};

use crate::cli::systems::SystemCommands;
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
            // Always use paged endpoint — GET /systems is deprecated
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
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/systems");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
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
            let path = format!("/org/{org}/project/{project}/system/{}/links/requirement/{}", args.id, args.requirement_id);
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
            let path = format!("/org/{org}/project/{project}/system/{}/links/testCase/{}", args.id, args.test_case_id);
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
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/systems/customFields/renameOption");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
