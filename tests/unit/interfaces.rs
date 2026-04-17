// tests/unit/interfaces.rs
use serde_json::json;

use flow_cli::cli::interfaces::{InterfaceCommands, InterfaceItemArgs};
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_interfaces;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn list_calls_get_on_interfaces_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_interfaces(
        InterfaceCommands::List(ctx("o", "p")),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/interfaces/paged");
}

#[tokio::test]
async fn create_calls_post_on_interfaces_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_interfaces(
        InterfaceCommands::Create(PatchCollectionArgs {
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
    assert_eq!(call.path, "/org/o/project/p/interfaces");
}

#[tokio::test]
async fn delete_calls_delete_on_interface_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_interfaces(
        InterfaceCommands::Delete(InterfaceItemArgs {
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
    assert_eq!(call.method, "DELETE");
    assert_eq!(call.path, "/org/o/project/p/interface/5");
}
