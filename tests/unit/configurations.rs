// tests/unit/configurations.rs
use crate::helpers::MockHttpClient;
use flow_cli::cli::ResourceContextArgs;
use flow_cli::cli::configurations::ConfigurationCommands;
use flow_cli::config::Config;
use flow_cli::handlers::handle_configurations;
use flow_cli::output::OutputFormat;
use serde_json::json;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

#[tokio::test]
async fn list_calls_get_on_configurations_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_configurations(
        ConfigurationCommands::List(ctx("o", "p")),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/configurations");
}
