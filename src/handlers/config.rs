// src/handlers/config.rs
use anyhow::Result;
use serde_json::json;
use std::path::Path;

use crate::cli::ConfigCommands;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

pub fn handle_config(
    command: ConfigCommands,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    match command {
        ConfigCommands::Show => print_output(&json!({
            "configPath": config_path.display().to_string(),
            "baseUrl": config.effective_base_url(),
            "org": config.effective_org(),
            "project": config.effective_project(),
            "hasAccessToken": std::env::var("FLOW_ACCESS_TOKEN").is_ok() || config.access_token.is_some(),
            "hasRefreshToken": config.refresh_token.is_some(),
            "hasBasicAuth": (std::env::var("FLOW_USERNAME").is_ok() && std::env::var("FLOW_PASSWORD").is_ok())
                || (config.username.is_some() && config.password.is_some()),
        }), OutputFormat::Json)?,
        ConfigCommands::Path => {
            println!("{}", config_path.display());
        }
        ConfigCommands::SetContext(args) => {
            if let Some(org) = args.org { config.org_alias = Some(org); }
            if let Some(project) = args.project { config.project_alias = Some(project); }
            if let Some(base_url) = args.base_url { config.base_url = Some(base_url); }
            config.save(config_path)?;
            print_output(&json!({
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
            }), OutputFormat::Json)?;
        }
    }
    Ok(())
}
