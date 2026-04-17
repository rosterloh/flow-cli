// tests/unit/values.rs
use serde_json::json;

use flow_cli::cli::values::{ValueCommands, ValueItemArgs};
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_values;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs { org: Some(org.into()), project: Some(project.into()) }
}

#[tokio::test]
async fn get_calls_get_on_value_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_values(
        ValueCommands::Get(ValueItemArgs { context: ctx("o", "p"), id: 99 }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/value/99");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_values(
        ValueCommands::SetImportId(PatchCollectionArgs {
            context: ctx("o", "p"),
            payload: JsonPayloadArgs { json: Some("{}".into()), body_file: None },
        }),
        &mock, &Config::default(), OutputFormat::Json,
    ).await.unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "PUT");
    assert_eq!(call.path, "/org/o/project/p/values/importid");
}
