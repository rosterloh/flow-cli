// src/handlers/test_plans.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::test_plans::TestPlanCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{build_links_wrapper, build_patch_single, load_json_payload, resolve_context};

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
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testPlan/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::Patch(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            if args.id.is_none() && (args.name.is_some() || args.description.is_some()) {
                anyhow::bail!("--id is required when using per-field flags");
            }
            let body = if let Some(id) = args.id {
                let mut fields = Vec::new();
                if let Some(name) = args.name {
                    fields.push(("name".to_string(), json!(name)));
                }
                if let Some(description) = args.description {
                    fields.push(("description".to_string(), json!(description)));
                }
                if fields.is_empty() {
                    anyhow::bail!("at least one field flag required (--name, --description)");
                }
                build_patch_single("testPlanId", json!(id), fields)
            } else {
                load_json_payload(&args.payload)?
            };
            let path = format!("/org/{org}/project/{project}/testPlans");
            let response = client
                .send(Method::PATCH, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
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
            let path = format!(
                "/org/{org}/project/{project}/testPlan/{}/testCycle",
                args.id
            );
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::SetImportId(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/testPlans/importid");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::LinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = match (args.test_plan_id, args.test_case_id) {
                (Some(pid), Some(tcid)) => {
                    build_links_wrapper(vec![json!({ "testPlanId": pid, "testCaseId": tcid })])
                }
                (Some(_), None) => anyhow::bail!("--test-case-id is required in flag mode"),
                (None, Some(_)) => anyhow::bail!("--test-plan-id is required in flag mode"),
                (None, None) => load_json_payload(&args.payload)?,
            };
            let path = format!("/org/{org}/project/{project}/link/testPlanTestCase");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        TestPlanCommands::LinkTestCaseCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/testPlanTestCase/crossProject");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
