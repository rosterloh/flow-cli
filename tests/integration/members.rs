// tests/integration/members.rs
use flow_cli::cli::members::{MemberCommands, OrgMemberArgs};
use flow_cli::client::FlowClient;
use flow_cli::config::Config;
use flow_cli::handlers::handle_members;
use flow_cli::output::OutputFormat;

use crate::require_credentials;

fn make_config(org: &str) -> Config {
    Config {
        org_alias: Some(org.into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn members_list_org_returns_without_error() {
    let Some((org, _project)) = require_credentials() else {
        return;
    };
    let config = make_config(&org);
    let client = FlowClient::from_config(&config).unwrap();
    handle_members(
        MemberCommands::ListOrg(OrgMemberArgs { org: Some(org) }),
        &client,
        &config,
        OutputFormat::Json,
    )
    .await
    .unwrap();
}
