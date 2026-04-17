// src/handlers/test_runs.rs
use super::{load_json_payload, resolve_context};
use crate::cli::test_runs::TestRunCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use reqwest::Method;

pub async fn handle_test_runs<C: HttpSend>(
    command: TestRunCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestRunCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestRunCommands::Patch(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client
                .send(Method::PATCH, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        TestRunCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}",
                args.cycle_id, args.id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestRunCommands::SetSteps(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/testCycle/{}/testRun/{}/steps",
                args.cycle_id, args.id
            );
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
