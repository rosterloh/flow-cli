// tests/unit/requirements.rs
use serde_json::json;

use flow_cli::cli::requirements::{
    ListRequirementsArgs, RequirementCommands, RequirementJiraArgs, RequirementPatchArgs,
    RequirementScope,
};
use flow_cli::cli::{
    ItemArgs, JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs,
};
use flow_cli::config::Config;
use flow_cli::handlers::handle_requirements;
use flow_cli::output::OutputFormat;

use crate::helpers::MockHttpClient;

fn ctx(org: &str, project: &str) -> ResourceContextArgs {
    ResourceContextArgs {
        org: Some(org.into()),
        project: Some(project.into()),
    }
}

fn list_args(org: &str, project: &str) -> ListArgs {
    ListArgs {
        context: ctx(org, project),
        paged: false,
        after: None,
        limit: None,
    }
}

#[tokio::test]
async fn list_default_calls_requirements_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: list_args("o", "p"),
            scope: None,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/requirements");
}

#[tokio::test]
async fn list_paged_calls_paged_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: ListArgs {
                context: ctx("o", "p"),
                paged: true,
                after: None,
                limit: None,
            },
            scope: None,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(mock.calls()[0].path, "/org/o/project/p/requirements/paged");
}

#[tokio::test]
async fn list_scope_org_calls_organization_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: list_args("o", "p"),
            scope: Some(RequirementScope::Org),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(
        mock.calls()[0].path,
        "/org/o/project/p/requirements/organization"
    );
}

#[tokio::test]
async fn list_scope_without_system_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::List(ListRequirementsArgs {
            list: list_args("o", "p"),
            scope: Some(RequirementScope::WithoutSystem),
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(
        mock.calls()[0].path,
        "/org/o/project/p/requirements/withoutSystem"
    );
}

#[tokio::test]
async fn filter_calls_post_on_filter_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::Filter(PatchCollectionArgs {
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
    assert_eq!(call.path, "/org/o/project/p/requirements/filter");
}

#[tokio::test]
async fn set_value_calls_put_on_requirements_value_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_requirements(
        RequirementCommands::SetValue(PatchCollectionArgs {
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
    assert_eq!(call.path, "/org/o/project/p/requirements/value");
}

#[tokio::test]
async fn list_test_cases_calls_correct_path() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::ListTestCases(ItemArgs {
            context: ctx("o", "p"),
            id: 42,
        }),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    assert_eq!(
        mock.calls()[0].path,
        "/org/o/project/p/requirement/42/testCases"
    );
}

#[tokio::test]
async fn link_jira_calls_post_on_jira_issues_path() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_requirements(
        RequirementCommands::LinkJira(RequirementJiraArgs {
            context: ctx("o", "p"),
            id: 10,
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
    assert_eq!(call.path, "/org/o/project/p/requirement/10/jiraIssues");
}

#[tokio::test]
async fn get_custom_fields_calls_get() {
    let mock = MockHttpClient::with_response(json!([]));
    handle_requirements(
        RequirementCommands::GetCustomFields(ctx("o", "p")),
        &mock,
        &Config::default(),
        OutputFormat::Json,
    )
    .await
    .unwrap();
    let call = &mock.calls()[0];
    assert_eq!(call.method, "GET");
    assert_eq!(call.path, "/org/o/project/p/requirements/customFields");
}

#[tokio::test]
async fn patch_flag_mode_builds_single_item_array() {
    let mock = MockHttpClient::with_response(json!({}));
    handle_requirements(
        RequirementCommands::Patch(RequirementPatchArgs {
            context: ctx("o", "p"),
            id: Some(2855),
            name: None,
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
    assert_eq!(call.path, "/org/o/project/p/requirements");
    assert_eq!(
        call.body.as_ref().unwrap(),
        &json!([{ "requirementId": 2855, "owner": "rio@skl.vc" }])
    );
}

#[tokio::test]
async fn patch_flag_mode_without_field_flags_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_requirements(
        RequirementCommands::Patch(RequirementPatchArgs {
            context: ctx("o", "p"),
            id: Some(2855),
            name: None,
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
}

#[tokio::test]
async fn patch_flag_mode_without_id_errors() {
    let mock = MockHttpClient::with_response(json!({}));
    let err = handle_requirements(
        RequirementCommands::Patch(RequirementPatchArgs {
            context: ctx("o", "p"),
            id: None,
            name: Some("orphaned".into()),
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
