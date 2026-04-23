// tests/integration.rs
#[path = "integration/configurations.rs"]
mod configurations;
#[path = "integration/documents.rs"]
mod documents;
#[path = "integration/interfaces.rs"]
mod interfaces;
#[path = "integration/members.rs"]
mod members;
#[path = "integration/requirements.rs"]
mod requirements;
#[path = "integration/systems.rs"]
mod systems;
#[path = "integration/test_cases.rs"]
mod test_cases;
#[path = "integration/test_cycles.rs"]
mod test_cycles;
#[path = "integration/test_plans.rs"]
mod test_plans;
#[path = "integration/test_runs.rs"]
mod test_runs;
#[path = "integration/values.rs"]
mod values;

/// Returns `(org, project)` from env vars when auth credentials are also
/// present, or `None` otherwise. Auth is satisfied by either `FLOW_ACCESS_TOKEN`
/// (bearer) or `FLOW_USERNAME` + `FLOW_PASSWORD` (basic) — `FlowClient` picks
/// whichever is set.
///
/// Example:
/// ```
/// let Some((org, project)) = require_credentials() else { return };
/// ```
pub fn require_credentials() -> Option<(String, String)> {
    let has_bearer = std::env::var("FLOW_ACCESS_TOKEN").is_ok();
    let has_basic =
        std::env::var("FLOW_USERNAME").is_ok() && std::env::var("FLOW_PASSWORD").is_ok();
    if !has_bearer && !has_basic {
        return None;
    }
    let org = std::env::var("FLOW_ORG").ok()?;
    let project = std::env::var("FLOW_PROJECT").ok()?;
    Some((org, project))
}
