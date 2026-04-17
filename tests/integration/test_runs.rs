// tests/integration/test_runs.rs
use crate::require_credentials;

#[tokio::test]
async fn test_runs_placeholder_skips_without_credentials() {
    let Some(_) = require_credentials() else { return };
    // No live test_runs endpoint exercised here; credentials gate ensures CI skips gracefully.
}
