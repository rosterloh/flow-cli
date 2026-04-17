// src/handlers/orgs.rs
use anyhow::Result;
use reqwest::Method;

use crate::cli::OrgsCommands;
use crate::client::HttpSend;
use crate::output::{OutputFormat, print_output};

pub async fn handle_orgs<C: HttpSend>(
    command: OrgsCommands,
    client: &C,
    output: OutputFormat,
) -> Result<()> {
    match command {
        OrgsCommands::List => {
            let response = client.send(Method::GET, "/orgs", &[], None, true).await?;
            print_output(&response, output)?;
        }
    }
    Ok(())
}
