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
