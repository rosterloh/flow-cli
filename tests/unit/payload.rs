// tests/unit/payload.rs
use serde_json::{Value, json};

use flow_cli::handlers::build_patch_single;

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
