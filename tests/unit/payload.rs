// tests/unit/payload.rs
use serde_json::{Value, json};

use flow_cli::handlers::build_links_wrapper;
use flow_cli::handlers::build_patch_single;
use flow_cli::handlers::build_system_link_item;

#[test]
fn build_patch_single_wraps_id_and_fields_in_array() {
    let fields = vec![
        ("owner".to_string(), json!("rio@skl.vc")),
        ("name".to_string(), json!("New Name")),
    ];
    let body: Value = build_patch_single("testCaseId", json!(326), fields);
    assert_eq!(
        body,
        json!([{ "testCaseId": 326, "owner": "rio@skl.vc", "name": "New Name" }])
    );
}

#[test]
fn build_patch_single_skips_no_fields() {
    let body: Value = build_patch_single("testCaseId", json!(326), vec![]);
    assert_eq!(body, json!([{ "testCaseId": 326 }]));
}

#[test]
fn build_links_wrapper_wraps_single_link_in_links_array() {
    let link = json!({ "requirementId": 2855, "testCaseId": 326 });
    let body = build_links_wrapper(vec![link]);
    assert_eq!(
        body,
        json!({ "links": [{ "requirementId": 2855, "testCaseId": 326 }] })
    );
}

#[test]
fn build_links_wrapper_accepts_empty_links() {
    let body = build_links_wrapper(vec![]);
    assert_eq!(body, json!({ "links": [] }));
}

#[test]
fn build_system_link_item_wraps_entity_in_array() {
    let body = build_system_link_item("testPlanId", json!(203));
    assert_eq!(body, json!([{ "testPlanId": 203 }]));
}

#[test]
fn build_system_link_item_supports_string_ids() {
    let body = build_system_link_item("documentId", json!("doc-uuid"));
    assert_eq!(body, json!([{ "documentId": "doc-uuid" }]));
}
