// src/handlers/test_cycles.rs
use anyhow::Result;
use reqwest::Method;
use crate::cli::test_cycles::TestCycleCommands;
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};
use super::resolve_context;

pub async fn handle_test_cycles<C: HttpSend>(
    command: TestCycleCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        TestCycleCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCycle/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        TestCycleCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCycle/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
