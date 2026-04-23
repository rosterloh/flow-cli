// tests/integration/systems.rs
use flow_cli::cli::systems::{ListSystemsArgs, SystemCommands};
use flow_cli::cli::{ListArgs, ResourceContextArgs};
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_systems;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(org: &str, project: &str) -> Config {
    Config {
        org_alias: Some(org.into()),
        project_alias: Some(project.into()),
        ..Default::default()
    }
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn systems_list_returns_without_error() {
    let Some((org, project)) = require_credentials() else {
        return;
    };
    let config = make_config(&org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_systems(
        SystemCommands::List(ListSystemsArgs {
            list: ListArgs {
                context: ctx(&org, &project),
                paged: true,
                after: None,
                limit: Some(5),
            },
            top_level: false,
        }),
        &client,
        &config,
        OutputFormat::Json,
    )
    .await
    .unwrap();
}
