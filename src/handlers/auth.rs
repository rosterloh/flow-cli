// src/handlers/auth.rs
use anyhow::{Result, anyhow};
use reqwest::Method;
use serde_json::{Value, json};
use std::path::Path;

use crate::cli::AuthCommands;
use crate::client::{FlowClient, HttpSend};
use crate::config::Config;
use crate::output::{OutputFormat, print_output};

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
            print_output(&response, OutputFormat::Json)?;
        }
        AuthCommands::SetBearer(args) => {
            let response = json!({
                "auth": "bearer",
                "saved": args.save,
                "accessTokenPreview": super::redact(&args.access_token),
            });
            if args.save {
                config.access_token = Some(args.access_token);
                config.username = None;
                config.password = None;
                config.save(config_path)?;
            }
            print_output(&response, OutputFormat::Json)?;
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
            print_output(&response, OutputFormat::Json)?;
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
            print_output(&json!({
                "auth": auth,
                "baseUrl": config.effective_base_url(),
                "org": config.effective_org(),
                "project": config.effective_project(),
                "configPath": config_path.display().to_string(),
                "savedRefreshToken": config.refresh_token.is_some(),
            }), OutputFormat::Json)?;
        }
        AuthCommands::Clear { all } => {
            config.access_token = None;
            config.username = None;
            config.password = None;
            if all {
                config.refresh_token = None;
            }
            config.save(config_path)?;
            print_output(&json!({
                "cleared": "auth",
                "refreshTokenCleared": all,
            }), OutputFormat::Json)?;
        }
    }
    Ok(())
}
