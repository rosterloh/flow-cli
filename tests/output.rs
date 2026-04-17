// tests/unit/output.rs
use serde_json::json;

use flow_cli::output::{OutputFormat, print_output};

#[test]
fn json_mode_succeeds_for_object() {
    let v = json!({"key": "value", "n": 42});
    assert!(print_output(&v, OutputFormat::Json).is_ok());
}

#[test]
fn json_mode_succeeds_for_array() {
    let v = json!([{"id": 1, "name": "foo"}, {"id": 2, "name": "bar"}]);
    assert!(print_output(&v, OutputFormat::Json).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_empty_array() {
    assert!(print_output(&json!([]), OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_array_of_objects() {
    let v = json!([{"id": 1, "name": "alpha"}, {"id": 2, "name": "beta"}]);
    assert!(print_output(&v, OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_object() {
    let v = json!({"status": 204, "message": "deleted"});
    assert!(print_output(&v, OutputFormat::Table).is_ok());
}

#[test]
fn table_mode_does_not_panic_on_scalar() {
    assert!(print_output(&json!("just a string"), OutputFormat::Table).is_ok());
}
