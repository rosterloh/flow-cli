// tests/integration/test_plans.rs
use flow_cli::cli::ResourceContextArgs;
use flow_cli::cli::test_plans::TestPlanCommands;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_plans;
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
async fn test_plans_list_returns_without_error() {
    let Some((org, project)) = require_credentials() else {
        return;
    };
    let config = make_config(&org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_test_plans(
        TestPlanCommands::List(ctx(&org, &project)),
        &client,
        &config,
        OutputFormat::Json,
    )
    .await
    .unwrap();
}
