// src/handlers/configurations.rs
use super::{load_json_payload, resolve_context};
use crate::cli::configurations::ConfigurationCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use reqwest::Method;

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
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/configurations");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
