// tests/unit/test_cycles.rs
use serde_json::json;
use flow_cli::cli::test_cycles::{TestCycleCommands, TestCycleItemArgs};
use flow_cli::cli::ResourceContextArgs;
use flow_cli::config::Config;
use flow_cli::handlers::handle_test_cycles;
use flow_cli::output::OutputFormat;
use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_test_cycle_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_test_cycles(TestCycleCommands::Get(TestCycleItemArgs { context: ctx("o", "p"), id: 42 }), &mock, &Config::default(), OutputFormat::Json).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/testCycle/42");
}

#[tokio::test]
async fn delete_calls_delete_on_test_cycle_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_test_cycles(TestCycleCommands::Delete(TestCycleItemArgs { context: ctx("o", "p"), id: 7 }), &mock, &Config::default(), OutputFormat::Json).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/testCycle/7");
}
