// src/handlers/requirements.rs
use anyhow::Result;
use reqwest::Method;
use serde_json::json;

use crate::cli::requirements::{RequirementCommands, RequirementScope};
use crate::client::HttpSend;
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

use super::{
    build_patch_single, list_query, load_json_payload, named_items_body, patch_collection,
    resolve_context,
};

pub async fn handle_requirements<C: HttpSend>(
    command: RequirementCommands,
    client: &C,
    config: &Config,
    output: OutputFormat,
) -> Result<()> {
    match command {
        RequirementCommands::List(args) => {
            let (org, project) = resolve_context(&args.list.context, config)?;
            let path = match args.scope {
                None if args.list.paged => {
                    format!("/org/{org}/project/{project}/requirements/paged")
                }
                None => format!("/org/{org}/project/{project}/requirements"),
                Some(RequirementScope::Org) => {
                    format!("/org/{org}/project/{project}/requirements/organization")
                }
                Some(RequirementScope::Project) => {
                    format!("/org/{org}/project/{project}/requirements/project")
                }
                Some(RequirementScope::WithoutSystem) => {
                    format!("/org/{org}/project/{project}/requirements/withoutSystem")
                }
            };
            let query = list_query(&args.list.after, args.list.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = named_items_body(args.names, args.description);
            let path = format!("/org/{org}/project/{project}/requirements");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Patch(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            if args.id.is_none() && (args.name.is_some() || args.owner.is_some()) {
                anyhow::bail!("--id is required when using per-field flags");
            }
            let body = if let Some(id) = args.id {
                let mut fields = Vec::new();
                if let Some(name) = args.name {
                    fields.push(("name".to_string(), json!(name)));
                }
                if let Some(owner) = args.owner {
                    fields.push(("owner".to_string(), json!(owner)));
                }
                if fields.is_empty() {
                    anyhow::bail!("at least one field flag required (--name, --owner)");
                }
                build_patch_single("requirementId", json!(id), fields)
            } else {
                load_json_payload(&args.payload)?
            };
            let path = format!("/org/{org}/project/{project}/requirements");
            let response = client
                .send(Method::PATCH, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Filter(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/filter");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::SetStage(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/stage");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::SetImportId(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/importid");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::SetValue(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/value");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::ListTestCases(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/testCases",
                args.id
            );
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::ListTestPlans(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/testPlans",
                args.id
            );
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UploadFile(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/uploadFile",
                args.id
            );
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UploadImage(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/imageUrl/{}",
                args.id, args.file_id
            );
            let response = client.send(Method::POST, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/jiraIssues",
                args.id
            );
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UnlinkJira(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/jiraIssues/{}",
                args.id, args.jira_issue_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Link(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}",
                args.id, args.link_type
            );
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Unlink(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}/{}",
                args.id, args.link_type, args.linked_requirement_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::UnlinkCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!(
                "/org/{org}/project/{project}/requirement/{}/links/{}/cross_project/{}/{}",
                args.id, args.link_type, args.linked_project, args.linked_requirement_id
            );
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkTestCase(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/link/requirementTestCase");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::LinkTestCaseCrossProject(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path =
                format!("/org/{org}/project/{project}/link/requirementTestCase/crossProject");
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::GetCustomFields(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/requirements/customFields");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_output(&response, output)?;
        }
        RequirementCommands::PatchCustomFields(args) => {
            patch_collection(client, config, args, output, |org, project| {
                format!("/org/{org}/project/{project}/requirements/customFields")
            })
            .await?;
        }
        RequirementCommands::RenameCustomFieldOption(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path =
                format!("/org/{org}/project/{project}/requirements/customFields/renameOption");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::AddConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/configuration");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::RemoveConfiguration(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = load_json_payload(&args.payload)?;
            let path = format!("/org/{org}/project/{project}/requirements/configuration");
            let response = client
                .send(Method::DELETE, &path, &[], Some(body), true)
                .await?;
            print_output(&response, output)?;
        }
        RequirementCommands::Search(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirements/filter");
            let all = client
                .send(Method::POST, &path, &[], Some(json!({})), true)
                .await?;
            let term = args.term.to_lowercase();
            let hits: Vec<_> = all
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter(|r| {
                    r.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&term)
                })
                .map(|r| {
                    json!({
                        "id": r["id"],
                        "name": r["name"],
                        "owner": r.get("owner").cloned().unwrap_or(serde_json::Value::Null),
                    })
                })
                .collect();
            if hits.is_empty() {
                println!("No requirements matched '{}'", args.term);
            } else {
                print_output(&serde_json::Value::Array(hits), output)?;
            }
        }
    }
    Ok(())
}
