// tests/unit/test_plans.rs
use serde_json::json;

use flow_cli::cli::test_plans::{
    TestPlanCommands, TestPlanItemArgs, TestPlanItemPayloadArgs, TestPlanPatchArgs,
};
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_plans;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn create_calls_post_on_test_plans_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_plans(
        TestPlanCommands::Create(PatchCollectionArgs {
            context: ctx("o", "p"),
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
    assert_eq!(call.path, "/org/o/project/p/testPlans");
}

#[tokio::test]
async fn get_calls_get_on_test_plan_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 5}));
    handle_test_plans(
        TestPlanCommands::Get(TestPlanItemArgs {
            context: ctx("o", "p"),
            id: 5,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testPlan/5");
}

#[tokio::test]
async fn delete_calls_delete_on_test_plan_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_plans(
        TestPlanCommands::Delete(TestPlanItemArgs {
            context: ctx("o", "p"),
            id: 3,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testPlan/3");
}

#[tokio::test]
async fn create_cycle_calls_post_on_test_cycle_path() {
    let mock = MockHttpClient::with_response(json!({"id": 10}));
    handle_test_plans(
        TestPlanCommands::CreateCycle(TestPlanItemPayloadArgs {
            context: ctx("o", "p"),
            id: 2,
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
    assert_eq!(call.path, "/org/o/project/p/testPlan/2/testCycle");
}

#[tokio::test]
async fn link_test_case_calls_put_on_link_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_plans(
        TestPlanCommands::LinkTestCase(PatchCollectionArgs {
            context: ctx("o", "p"),
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
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/link/testPlanTestCase");
}

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_plans(
        TestPlanCommands::Patch(TestPlanPatchArgs {
            context: ctx("o", "p"),
            id: Some(202),
            name: Some("Internal Network Performance Test".into()),
            description: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PATCH");
    assert_eq!(call.path, "/org/o/project/p/testPlans");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testPlanId": 202, "name": "Internal Network Performance Test" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_plans(
        TestPlanCommands::Patch(TestPlanPatchArgs {
            context: ctx("o", "p"),
            id: Some(202),
            name: None,
            description: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("at least one field flag"));
}

#[tokio::test]
async fn patch_flag_mode_without_id_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_plans(
        TestPlanCommands::Patch(TestPlanPatchArgs {
            context: ctx("o", "p"),
            id: None,
            name: Some("orphaned".into()),
            description: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("--id is required"));
    assert!(mock.calls().is_empty());
}
