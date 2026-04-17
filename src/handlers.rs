use anyhow::{Result, anyhow, bail};
use reqwest::Method;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

use crate::cli::{
    AuthCommands, ConfigCommands, JsonPayloadArgs, OrgsCommands, PatchCollectionArgs,
    ProjectCommands, RawCommand, RequirementCommands, ResourceContextArgs, SystemCommands,
    TestCaseCommands, TestPlanCommands, UtilCommands, ValueCommands,
};
use crate::client::FlowClient;
use crate::config::Config;

pub async fn handle_auth(
    command: AuthCommands,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    match command {
        AuthCommands::Exchange(args) => {
            let refresh_token = args
                .refresh_token
                .or_else(|| config.refresh_token.clone())
                .ok_or_else(|| {
                    anyhow!("no refresh token provided; pass --refresh-token or save one first")
                })?;
            let client = FlowClient::exchange_client(config)?;
            let body = json!({ "refreshToken": refresh_token });
            let response = client
                .send(Method::POST, "/auth/exchange", &[], Some(body), false)
                .await?;

            if args.save || args.save_refresh_token {
                config.access_token = response
                    .get("accessToken")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                if args.save_refresh_token {
                    config.refresh_token = Some(
                        response
                            .get("refreshToken")
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned)
                            .unwrap_or_else(|| refresh_token.clone()),
                    );
                }
                config.save(config_path)?;
            }

            print_json(&response)?;
        }
        AuthCommands::SetBearer(args) => {
            let response = json!({
                "auth": "bearer",
                "saved": args.save,
                "accessTokenPreview": redact(&args.access_token),
            });
            if args.save {
                config.access_token = Some(args.access_token);
                config.username = None;
                config.password = None;
                config.save(config_path)?;
            }
            print_json(&response)?;
        }
        AuthCommands::SetBasic(args) => {
            let response = json!({
                "auth": "basic",
                "saved": args.save,
                "username": args.username,
            });
            if args.save {
                config.username = response
                    .get("username")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned);
                config.password = Some(args.password);
                config.access_token = None;
                config.save(config_path)?;
            }
            print_json(&response)?;
        }
        AuthCommands::Status => {
            let auth =
                if std::env::var("FLOW_ACCESS_TOKEN").is_ok() || config.access_token.is_some() {
                    "bearer"
                } else if (std::env::var("FLOW_USERNAME").is_ok()
                    && std::env::var("FLOW_PASSWORD").is_ok())
                    || (config.username.is_some() && config.password.is_some())
                {
                    "basic"
                } else {
                    "none"
                };

            print_json(&json!({
                "auth": auth,
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
                "configPath": config_path.display().to_string(),
                "savedRefreshToken": config.refresh_token.is_some(),
            }))?;
        }
        AuthCommands::Clear { all } => {
            config.access_token = None;
            config.username = None;
            config.password = None;
            if all {
                config.refresh_token = None;
            }
            config.save(config_path)?;
            print_json(&json!({
                "cleared": "auth",
                "refreshTokenCleared": all,
            }))?;
        }
    }

    Ok(())
}

pub fn handle_config(
    command: ConfigCommands,
    config: &mut Config,
    config_path: &Path,
) -> Result<()> {
    match command {
        ConfigCommands::Show => print_json(&json!({
            "configPath": config_path.display().to_string(),
            "baseUrl": config.effective_base_url(),
            "org": config.effective_org(),
            "project": config.effective_project(),
            "hasAccessToken": std::env::var("FLOW_ACCESS_TOKEN").is_ok() || config.access_token.is_some(),
            "hasRefreshToken": config.refresh_token.is_some(),
            "hasBasicAuth": (std::env::var("FLOW_USERNAME").is_ok() && std::env::var("FLOW_PASSWORD").is_ok())
                || (config.username.is_some() && config.password.is_some()),
        }))?,
        ConfigCommands::Path => {
            println!("{}", config_path.display());
        }
        ConfigCommands::SetContext(args) => {
            if let Some(org) = args.org {
                config.org_alias = Some(org);
            }
            if let Some(project) = args.project {
                config.project_alias = Some(project);
            }
            if let Some(base_url) = args.base_url {
                config.base_url = Some(base_url);
            }
            config.save(config_path)?;
            print_json(&json!({
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
            }))?;
        }
    }

    Ok(())
}

pub async fn handle_orgs(command: OrgsCommands, client: &FlowClient) -> Result<()> {
    match command {
        OrgsCommands::List => {
            let response = client.send(Method::GET, "/orgs", &[], None, true).await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_projects(
    command: ProjectCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        ProjectCommands::List(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/projects");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_json(&response)?;
        }
        ProjectCommands::Create(args) => {
            let org = resolve_org(&args.org, config)?;
            let path = format!("/org/{org}/projects");
            let response = client
                .send(
                    Method::POST,
                    &path,
                    &[],
                    Some(json!({ "name": args.name })),
                    true,
                )
                .await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_requirements(
    command: RequirementCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        RequirementCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/requirements/paged")
            } else {
                format!("/org/{org}/project/{project}/requirements")
            };
            let query = list_query(&args.after, args.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_json(&response)?;
        }
        RequirementCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_json(&response)?;
        }
        RequirementCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = named_items_body(args.names, args.description);
            let path = format!("/org/{org}/project/{project}/requirements");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_json(&response)?;
        }
        RequirementCommands::Patch(args) => {
            patch_collection(client, config, args, |org, project| {
                format!("/org/{org}/project/{project}/requirements")
            })
            .await?;
        }
        RequirementCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/requirement/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_systems(
    command: SystemCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        SystemCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/systems/paged")
            } else {
                format!("/org/{org}/project/{project}/systems")
            };
            let query = list_query(&args.after, args.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_json(&response)?;
        }
        SystemCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let mut body = json!({ "name": args.name });
            if let Some(description) = args.description {
                body["description"] = Value::String(description);
            }
            if let Some(owner) = args.owner {
                body["owner"] = Value::String(owner);
            }
            if let Some(parent_id) = args.parent_id {
                body["parentId"] = Value::String(parent_id);
            }
            if let Some(prefix) = args.prefix {
                body["prefix"] = Value::String(prefix);
            }
            let path = format!("/org/{org}/project/{project}/system");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_json(&response)?;
        }
        SystemCommands::Update(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = if args.payload.json.is_some() || args.payload.body_file.is_some() {
                load_json_payload(&args.payload)?
            } else {
                let mut body = json!({});
                if let Some(name) = args.name {
                    body["name"] = Value::String(name);
                }
                if let Some(description) = args.description {
                    body["description"] = Value::String(description);
                }
                if let Some(owner) = args.owner {
                    body["owner"] = Value::String(owner);
                }
                if let Some(parent_id) = args.parent_id {
                    body["parentId"] = Value::String(parent_id);
                }
                if let Some(prefix) = args.prefix {
                    body["prefix"] = Value::String(prefix);
                }
                body
            };
            let path = format!("/org/{org}/project/{project}/system/{}", args.id);
            let response = client
                .send(Method::PUT, &path, &[], Some(body), true)
                .await?;
            print_json(&response)?;
        }
        SystemCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/system/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_test_cases(
    command: TestCaseCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        TestCaseCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = if args.paged {
                format!("/org/{org}/project/{project}/testCases/paged")
            } else {
                format!("/org/{org}/project/{project}/testCases")
            };
            let query = list_query(&args.after, args.limit);
            let response = client.send(Method::GET, &path, &query, None, true).await?;
            print_json(&response)?;
        }
        TestCaseCommands::Get(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_json(&response)?;
        }
        TestCaseCommands::Create(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let body = named_items_body(args.names, args.description);
            let path = format!("/org/{org}/project/{project}/testCases");
            let response = client
                .send(Method::POST, &path, &[], Some(body), true)
                .await?;
            print_json(&response)?;
        }
        TestCaseCommands::Patch(args) => {
            patch_collection(client, config, args, |org, project| {
                format!("/org/{org}/project/{project}/testCases")
            })
            .await?;
        }
        TestCaseCommands::Delete(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/testCase/{}", args.id);
            let response = client.send(Method::DELETE, &path, &[], None, true).await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_test_plans(
    command: TestPlanCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        TestPlanCommands::List(args) => {
            let (org, project) = resolve_context(&args, config)?;
            let path = format!("/org/{org}/project/{project}/testPlans");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_json(&response)?;
        }
        TestPlanCommands::Patch(args) => {
            patch_collection(client, config, args, |org, project| {
                format!("/org/{org}/project/{project}/testPlans")
            })
            .await?;
        }
    }

    Ok(())
}

pub async fn handle_values(
    command: ValueCommands,
    client: &FlowClient,
    config: &Config,
) -> Result<()> {
    match command {
        ValueCommands::List(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let suffix = if args.numeric {
                "values/number"
            } else {
                "values"
            };
            let path = format!("/org/{org}/project/{project}/{suffix}");
            let response = client.send(Method::GET, &path, &[], None, true).await?;
            print_json(&response)?;
        }
        ValueCommands::SetNumber(args) => {
            let (org, project) = resolve_context(&args.context, config)?;
            let path = format!("/org/{org}/project/{project}/value/{}/number", args.id);
            let response = client
                .send(
                    Method::PUT,
                    &path,
                    &[],
                    Some(json!({ "value": args.value })),
                    true,
                )
                .await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_util(command: UtilCommands, client: &FlowClient) -> Result<()> {
    match command {
        UtilCommands::ConvertHtml(args) => {
            let body = Value::Array(
                args.html
                    .into_iter()
                    .map(|html| json!({ "html": html }))
                    .collect(),
            );
            let response = client
                .send(Method::POST, "/util/convert-html", &[], Some(body), true)
                .await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

pub async fn handle_raw(command: RawCommand, client: &FlowClient) -> Result<()> {
    let query = command
        .query
        .iter()
        .map(|entry| parse_query_pair(entry))
        .collect::<Result<Vec<_>>>()?;
    let body = load_optional_json_payload(&command.payload)?;
    let response = client
        .send(
            command.method.as_method(),
            &command.path,
            &query,
            body,
            true,
        )
        .await?;
    print_json(&response)?;
    Ok(())
}

fn resolve_context(args: &ResourceContextArgs, config: &Config) -> Result<(String, String)> {
    let org = resolve_org(&args.org, config)?;
    let project = args
        .project
        .clone()
        .or_else(|| config.effective_project())
        .ok_or_else(|| anyhow!("no project configured; pass --project or set one with `flow-cli config set-context --project ...`"))?;
    Ok((org, project))
}

fn resolve_org(org: &Option<String>, config: &Config) -> Result<String> {
    org.clone()
        .or_else(|| config.effective_org())
        .ok_or_else(|| anyhow!("no org configured; pass --org or set one with `flow-cli config set-context --org ...`"))
}

fn list_query(after: &Option<String>, limit: Option<u32>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(after) = after {
        query.push(("after".to_string(), after.clone()));
    }
    if let Some(limit) = limit {
        query.push(("limit".to_string(), limit.to_string()));
    }
    query
}

fn load_json_payload(args: &JsonPayloadArgs) -> Result<Value> {
    load_optional_json_payload(args)?
        .ok_or_else(|| anyhow!("request body required; pass --json or --body-file"))
}

fn load_optional_json_payload(args: &JsonPayloadArgs) -> Result<Option<Value>> {
    match (&args.json, &args.body_file) {
        (Some(raw), None) => Ok(Some(parse_json(raw)?)),
        (None, Some(path)) => {
            let contents = fs::read_to_string(path)
                .map_err(|err| anyhow!("failed to read {}: {err}", path.display()))?;
            Ok(Some(parse_json(&contents)?))
        }
        (None, None) => Ok(None),
        _ => bail!("pass only one of --json or --body-file"),
    }
}

fn parse_json(raw: &str) -> Result<Value> {
    serde_json::from_str(raw).map_err(|_| anyhow!("invalid JSON payload"))
}

fn parse_query_pair(input: &str) -> Result<(String, String)> {
    let (key, value) = input
        .split_once('=')
        .ok_or_else(|| anyhow!("invalid query pair `{input}`, expected KEY=VALUE"))?;
    Ok((key.to_string(), value.to_string()))
}

fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn redact(value: &str) -> String {
    let suffix = value
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    if value.len() <= 4 {
        "****".to_string()
    } else {
        format!("***{suffix}")
    }
}

fn named_items_body(names: Vec<String>, description: Option<String>) -> Value {
    Value::Array(
        names
            .into_iter()
            .map(|name| {
                let mut item = json!({ "name": name });
                if let Some(description) = &description {
                    item["description"] = Value::String(description.clone());
                }
                item
            })
            .collect(),
    )
}

async fn patch_collection<F>(
    client: &FlowClient,
    config: &Config,
    args: PatchCollectionArgs,
    path: F,
) -> Result<()>
where
    F: FnOnce(&str, &str) -> String,
{
    let (org, project) = resolve_context(&args.context, config)?;
    let body = load_json_payload(&args.payload)?;
    let response = client
        .send(Method::PATCH, &path(&org, &project), &[], Some(body), true)
        .await?;
    print_json(&response)?;
    Ok(())
}
