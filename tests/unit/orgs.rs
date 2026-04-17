// tests/orgs.rs
use serde_json::json;

use flow_cli::cli::OrgsCommands;
use flow_cli::handlers::handle_orgs;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

#[tokio::test]
async fn orgs_list_calls_get_on_orgs_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_orgs(OrgsCommands::List, &mock, OutputFormat::Json)
        .await
        .unwrap();
    let calls = mock.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].method, "GET");
    assert_eq!(calls[0].path, "/orgs");
}
