// tests/unit/test_runs.rs
use serde_json::json;
use flow_cli::cli::test_runs::{TestRunCommands, TestRunItemArgs, TestRunItemPayloadArgs};
use flow_cli::cli::{JsonPayloadArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_runs;
use flow_cli::output::OutputFormat;
use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_runs(TestRunCommands::Get(TestRunItemArgs { context: ctx("o", "p"), cycle_id: 10, id: 1 }), &mock, &Config::default(), OutputFormat::Json).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCycle/10/testRun/1");
}

#[tokio::test]
async fn delete_calls_delete_on_test_run_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_runs(TestRunCommands::Delete(TestRunItemArgs { context: ctx("o", "p"), cycle_id: 10, id: 2 }), &mock, &Config::default(), OutputFormat::Json).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testCycle/10/testRun/2");
}

#[tokio::test]
async fn set_steps_calls_put_on_steps_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_test_runs(TestRunCommands::SetSteps(TestRunItemPayloadArgs { context: ctx("o", "p"), cycle_id: 5, id: 3, payload: JsonPayloadArgs { json: Some("[]".into()), body_file: None } }), &mock, &Config::default(), OutputFormat::Json).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/testCycle/5/testRun/3/steps");
}
