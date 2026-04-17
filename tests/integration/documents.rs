// tests/integration/documents.rs
use flow_cli::cli::documents::DocumentCommands;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_documents;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(token: &str, org: &str, project: &str) -> Config {
    let mut c = Config::default();
    c.access_token = Some(token.into());
    c.org_alias = Some(org.into());
    c.project_alias = Some(project.into());
    c
}

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn documents_list_returns_without_error() {
    let Some((token, org, project)) = require_credentials() else { return };
    let config = make_config(&token, &org, &project);
    let client = FlowClient::from_config(&config).unwrap();
    handle_documents(
        DocumentCommands::List(ctx(&org, &project)),
        &client, &config, OutputFormat::Json,
    ).await.unwrap();
}
