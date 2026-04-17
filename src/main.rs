// src/main.rs
use anyhow::Result;
use clap::Parser;

use flow_cli::cli::{Cli, Commands};
use flow_cli::client::FlowClient;
use flow_cli::config::{Config, config_path};
use flow_cli::handlers;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = cli.output;
    let config_path = config_path()?;
    let mut config = Config::load(&config_path)?;

    match cli.command {
        Commands::Auth { command } => {
            handlers::handle_auth(command, &mut config, &config_path).await?
        }
        Commands::Config { command } => {
            handlers::handle_config(command, &mut config, &config_path)?
        }
        Commands::Orgs { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_orgs(command, &client, output).await?;
        }
        Commands::Projects { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_projects(command, &client, &config, output).await?;
        }
        Commands::Configurations { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_configurations(command, &client, &config, output).await?;
        }
        Commands::Requirements { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_requirements(command, &client, &config, output).await?;
        }
        Commands::Systems { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_systems(command, &client, &config, output).await?;
        }
        Commands::TestCases { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_cases(command, &client, &config, output).await?;
        }
        Commands::TestPlans { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_plans(command, &client, &config, output).await?;
        }
        Commands::Documents { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_documents(command, &client, &config, output).await?;
        }
        Commands::Interfaces { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_interfaces(command, &client, &config, output).await?;
        }
        Commands::Members { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_members(command, &client, &config, output).await?;
        }
        Commands::Values { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_values(command, &client, &config, output).await?;
        }
        Commands::Util { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_util(command, &client, output).await?;
        }
        Commands::Raw(command) => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_raw(command, &client, output).await?;
        }
    }

    Ok(())
}
