// tests/unit/documents.rs
use serde_json::json;

use flow_cli::cli::documents::{DocumentCommands, DocumentItemArgs};
use flow_cli::cli::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};
use flow_cli::config::Config;
use flow_cli::handlers::handle_documents;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn list_calls_get_on_documents_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_documents(
        DocumentCommands::List(ctx("o", "p")),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/documents/paged");
}

#[tokio::test]
async fn get_calls_get_on_document_id_path() {
    let mock = MockHttpClient::with_response(json!({"id": 1}));
    handle_documents(
        DocumentCommands::Get(DocumentItemArgs {
            context: ctx("o", "p"),
            id: 7,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/document/7");
}

#[tokio::test]
async fn create_calls_post_on_documents_path() {
    let mock = MockHttpClient::with_response(json!({"id": 2}));
    handle_documents(
        DocumentCommands::Create(PatchCollectionArgs {
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
    assert_eq!(call.path, "/org/o/project/p/documents");
}

#[tokio::test]
async fn delete_calls_delete_on_document_id_path() {
    let mock = MockHttpClient::with_response(json!({"status": 204}));
    handle_documents(
        DocumentCommands::Delete(DocumentItemArgs {
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
    assert_eq!(call.path, "/org/o/project/p/document/3");
}

#[tokio::test]
async fn set_import_id_calls_put_on_importid_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_documents(
        DocumentCommands::SetImportId(PatchCollectionArgs {
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
    assert_eq!(call.path, "/org/o/project/p/documents/importid");
}
