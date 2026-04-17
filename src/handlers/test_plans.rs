// src/handlers/test_plans.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::test_plans::TestPlanCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{load_json_payload, patch_collection, resolve_context};

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
        TestPlanCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testPlans");
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            }).await?;
        }
        TestPlanCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::CreateCycle(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}/testCycle", args.id);
            let response = client.send(Method::POST, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::SetImportId(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testPlans/importid");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::LinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/testPlanTestCase");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::LinkTestCaseCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/testPlanTestCase/crossProject");
            let response = client.send(Method::PUT, &path, &[], Some(body), true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
