// tests/integration.rs
#[path = "integration/requirements.rs"]
mod requirements;
#[path = "integration/systems.rs"]
mod systems;
#[path = "integration/test_cases.rs"]
mod test_cases;
#[path = "integration/test_plans.rs"]
mod test_plans;

/// Returns `(token, org, project)` from env vars, or `None` if any are absent.
/// In local dev: call this at the top of every test and `return` early if None.
/// On CI: the workflow validates credentials before running this binary.
///
/// Example:
/// ```
/// let Some((token, org, project)) = require_credentials() else { return };
/// ```
pub fn require_credentials() -> Option<(String, String, String)> {
    let token = std::env::var("FLOW_ACCESS_TOKEN").ok()?;
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((token, org, project))
}
