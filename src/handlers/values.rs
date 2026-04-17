// src/handlers/values.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::values::ValueCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, resolve_context};

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
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/values/importid");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
