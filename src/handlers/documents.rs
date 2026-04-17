// src/handlers/documents.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::documents::DocumentCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, resolve_context};

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
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/documents");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        DocumentCommands::Patch(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/documents");
            let response = client
                .send(Method::PATCH, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        DocumentCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/document/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        DocumentCommands::SetImportId(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/documents/importid");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
