// tests/unit/systems.rs
use serde_json::json;

use flow_cli::cli::systems::{
    ListSystemsArgs, SystemCommands, SystemItemArgs, SystemLinkPayloadArgs, SystemLinkTestPlanArgs,
    SystemUnlinkTestCaseArgs,
};
use flow_cli::cli::{JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_systems;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn list_always_calls_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_systems(
        SystemCommands::List(ListSystemsArgs {
            list: ListArgs {
                context: ctx("o", "p"),
                paged: false,
                after: None,
                limit: None,
            },
            top_level: false,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/systems/paged");
}

#[tokio::test]
async fn bulk_update_calls_put_on_systems_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_systems(
        SystemCommands::BulkUpdate(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs {
                json: Some("[]".into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/systems");
}

#[tokio::test]
async fn list_documents_calls_get_on_links_documents_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_systems(
        SystemCommands::ListDocuments(SystemItemArgs {
            context: ctx("o", "p"),
            id: "sys-1".into(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/system/sys-1/links/documents");
}

#[tokio::test]
async fn link_requirement_calls_post_on_links_requirements_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkRequirement(SystemLinkPayloadArgs {
            context: ctx("o", "p"),
            id: "sys-1".into(),
            payload: JsonPayloadArgs {
                json: Some("{}".into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(
        call.path,
        "/org/o/project/p/system/sys-1/links/requirements"
    );
}

#[tokio::test]
async fn unlink_test_case_calls_delete_on_correct_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::UnlinkTestCase(SystemUnlinkTestCaseArgs {
            context: ctx("o", "p"),
            id: "sys-1".into(),
            test_case_id: 99,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/system/sys-1/links/testCase/99");
}

#[tokio::test]
async fn link_test_plan_flag_mode_builds_bare_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkTestPlan(SystemLinkTestPlanArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            test_plan_id: Some(203),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "POST");
    assert_eq!(
        call.path,
        "/org/o/project/p/system/sys-uuid/links/testPlans"
    );
    assert_eq!(call.body.as_ref().unwrap(), &json!([{ "testPlanId": 203 }]));
}

#[tokio::test]
async fn link_test_plan_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_systems(
        SystemCommands::LinkTestPlan(SystemLinkTestPlanArgs {
            context: ctx("o", "p"),
            id: "sys-uuid".into(),
            test_plan_id: None,
            payload: JsonPayloadArgs {
                json: Some(r#"[{"testPlanId": 9}]"#.into()),
                body_file: None,
            },
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(
        mock.calls()[0].body.as_ref().unwrap(),
        &json!([{"testPlanId": 9}])
    );
}
