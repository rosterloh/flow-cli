use anyhow::Result;
use clap::Parser;

use flow_cli::cli::{Cli, Commands};
use flow_cli::client::FlowClient;
use flow_cli::config::{Config, config_path};
use flow_cli::handlers;
use flow_cli::output::OutputFormat;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
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
            handlers::handle_orgs(command, &client, OutputFormat::default()).await?;
        }
        Commands::Projects { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_projects(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::Requirements { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_requirements(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::Systems { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_systems(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::TestCases { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_cases(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::TestPlans { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_test_plans(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::Values { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_values(command, &client, &config, OutputFormat::default()).await?;
        }
        Commands::Util { command } => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_util(command, &client, OutputFormat::default()).await?;
        }
        Commands::Raw(command) => {
            let client = FlowClient::from_config(&config)?;
            handlers::handle_raw(command, &client, OutputFormat::default()).await?;
        }
    }

    Ok(())
}
