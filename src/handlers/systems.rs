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
