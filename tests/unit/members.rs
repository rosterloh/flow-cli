// tests/unit/members.rs
use serde_json::json;

use flow_cli::cli::members::{MemberCommands, OrgMemberArgs, OrgRemoveMemberArgs, ProjectAddMemberArgs, ProjectMemberArgs};
use flow_cli::cli::{JsonPayloadArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_members;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn list_org_calls_get_on_org_members_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_members(
        MemberCommands::ListOrg(OrgMemberArgs { org: Some("my-org".into()) }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/my-org/members");
}

#[tokio::test]
async fn remove_org_calls_delete_on_member_email_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_members(
        MemberCommands::RemoveOrg(OrgRemoveMemberArgs {
            org: Some("my-org".into()),
            email: "user@example.com".into(),
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/my-org/members/user@example.com");
}

#[tokio::test]
async fn list_project_calls_get_on_project_members_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_members(
        MemberCommands::ListProject(ProjectMemberArgs { context: ctx("o", "p") }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/members");
}

#[tokio::test]
async fn add_project_calls_post_on_project_members_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_members(
        MemberCommands::AddProject(ProjectAddMemberArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(call.path, "/org/o/project/p/members");
}
