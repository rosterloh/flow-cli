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
                .send(
                    Method::POST,
                    &path,
                    &[],
                    Some(json!({ "name": args.name })),
                    true,
                )
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
