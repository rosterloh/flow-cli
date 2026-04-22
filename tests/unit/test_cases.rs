// tests/unit/test_cases.rs
use std::io::Write;

use serde_json::json;

use flow_cli::cli::test_cases::{
    TestCaseCommands, TestCaseItemArgs, TestCaseItemPayloadArgs, TestCasePatchArgs,
    TestCaseSetStepsArgs,
};
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_cases;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn set_steps_calls_put_on_steps_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseSetStepsArgs {
            context: ctx("o", "p"),
            id: 7,
            steps_file: None,
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
    assert_eq!(call.path, "/org/o/project/p/testCase/7/steps");
}

#[tokio::test]
async fn set_import_id_calls_patch_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetImportId(PatchCollectionArgs {
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
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/testCases/importid");
}

#[tokio::test]
async fn create_test_run_calls_post_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_cases(
        TestCaseCommands::CreateTestRun(TestCaseItemPayloadArgs {
            context: ctx("o", "p"),
            id: 5,
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
    assert_eq!(call.path, "/org/o/project/p/testCase/5/testRun");
}

#[tokio::test]
async fn get_custom_fields_calls_get_on_custom_fields_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_test_cases(
        TestCaseCommands::GetCustomFields(ctx("o", "p")),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCases/customFields");
}

#[tokio::test]
async fn list_requirements_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_test_cases(
        TestCaseCommands::ListRequirements(TestCaseItemArgs {
            context: ctx("o", "p"),
            id: 3,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(
        mock.calls()[0].path,
        "/org/o/project/p/testCase/3/links/requirements"
    );
}

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: Some(326),
            name: None,
            description: None,
            owner: Some("rio@skl.vc".into()),
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
    assert_eq!(call.path, "/org/o/project/p/testCases");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testCaseId": 326, "owner": "rio@skl.vc" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: Some(326),
            name: None,
            description: None,
            owner: None,
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains("at least one field flag"));
    assert!(mock.calls().is_empty());
}

#[tokio::test]
async fn patch_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: None,
            name: None,
            description: None,
            owner: None,
            payload: JsonPayloadArgs {
                json: Some(r#"[{"testCaseId": 1, "name": "x"}]"#.into()),
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
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "testCaseId": 1, "name": "x" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_id_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_test_cases(
        TestCaseCommands::Patch(TestCasePatchArgs {
            context: ctx("o", "p"),
            id: None,
            name: Some("orphaned".into()),
            description: None,
            owner: None,
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

#[tokio::test]
async fn set_steps_file_mode_wraps_in_server_shape() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    writeln!(
        f,
        r#"[
      {{"action": "do X", "expected": "Y"}},
      {{"action": "do A", "expected": "B"}}
    ]"#
    )
    .unwrap();

    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseSetStepsArgs {
            context: ctx("o", "p"),
            id: 7,
            steps_file: Some(f.path().to_path_buf()),
            payload: JsonPayloadArgs::default(),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/testCase/7/steps");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{
            "testCaseId": 7,
            "steps": [
                {"action": "do X", "expected": "Y"},
                {"action": "do A", "expected": "B"}
            ]
        }])
    );
}

#[tokio::test]
async fn set_steps_json_mode_still_works() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_cases(
        TestCaseCommands::SetSteps(TestCaseSetStepsArgs {
            context: ctx("o", "p"),
            id: 7,
            steps_file: None,
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
    assert_eq!(mock.calls()[0].body.as_ref().unwrap(), &json!([]));
}
