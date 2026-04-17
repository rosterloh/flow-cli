// src/handlers/test_plans.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::TestPlanCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{patch_collection, resolve_context};

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
        TestPlanCommands::Patch(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            }).await?;
        }
    }
    Ok(())
}
