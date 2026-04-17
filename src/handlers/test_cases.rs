// src/handlers/test_cases.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_cases::TestCaseCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{list_query, load_json_payload, named_items_body, patch_collection, resolve_context};

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
            }).await?;
        }
        TestCaseCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetSteps(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/steps", args.id);
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetImportId(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/importid")
            }).await?;
        }
        TestCaseCommands::UploadFile(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/file/{}", args.id, args.file_id);
            let response = client.send(Method::POST, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::LinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/jiraIssues", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::UnlinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/jiraIssues/{}", args.id, args.jira_issue_id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::ListRequirements(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/links/requirements", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::GetCustomFields(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/testCases/customFields");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::PatchCustomFields(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testCases/customFields")
            }).await?;
        }
        TestCaseCommands::RenameCustomFieldOption(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCases/customFields/renameOption");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::AddConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCases/configuration");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::RemoveConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCases/configuration");
            let response = client.send(Method::DELETE, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::SetStages(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCaseStages");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestCaseCommands::CreateTestRun(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}/testRun", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
