use anyhow::{Context, Result, bail};
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};

use crate::config::Config;

#[derive(Clone)]
pub struct FlowClient {
    client: Client,
    base_url: String,
    auth: Auth,
}

#[derive(Clone)]
enum Auth {
    Bearer(String),
    Basic { username: String, password: String },
}

impl FlowClient {
    pub fn from_config(config: &Config) -> Result<Self> {
        let access_token = std::env::var("FLOW_ACCESS_TOKEN")
            .ok()
            .or_else(|| config.access_token.clone());
        let username = std::env::var("FLOW_USERNAME")
            .ok()
            .or_else(|| config.username.clone());
        let password = std::env::var("FLOW_PASSWORD")
            .ok()
            .or_else(|| config.password.clone());

        let auth = if let Some(token) = access_token {
            Auth::Bearer(token)
        } else if let (Some(username), Some(password)) = (username, password) {
            Auth::Basic { username, password }
        } else {
            bail!(
                "no auth configured; use `flow-cli auth exchange`, `flow-cli auth set-bearer`, or `flow-cli auth set-basic`"
            );
        };

        Ok(Self {
            client: Client::builder().build()?,
            base_url: config.effective_base_url(),
            auth,
        })
    }

    pub fn exchange_client(config: &Config) -> Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            base_url: config.effective_base_url(),
            auth: Auth::Bearer(String::new()),
        })
    }

    pub async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
        with_auth: bool,
    ) -> Result<Value> {
        let url = format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            normalize_path(path)
        );
        let mut req = self.client.request(method, url).query(query);

        if with_auth {
            req = match &self.auth {
                Auth::Bearer(token) => req.bearer_auth(token),
                Auth::Basic { username, password } => req.basic_auth(username, Some(password)),
            };
        }

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req.send().await.context("request failed")?;
        let status = response.status();

        if status == StatusCode::NO_CONTENT {
            return Ok(json!({ "status": status.as_u16() }));
        }

        let bytes = response
            .bytes()
            .await
            .context("failed to read response body")?;
        if bytes.is_empty() {
            return Ok(json!({ "status": status.as_u16() }));
        }

        let parsed = serde_json::from_slice::<Value>(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));

        if !status.is_success() {
            bail!(
                "request failed with {}: {}",
                status.as_u16(),
                serde_json::to_string_pretty(&parsed)?
            );
        }

        Ok(parsed)
    }
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}
