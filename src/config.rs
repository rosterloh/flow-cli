use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_BASE_URL: &str = "https://api.flowengineering.com/rest/v1";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub org_alias: Option<String>,
    #[serde(default)]
    pub project_alias: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read config from {}", path.display()))?;
        Ok(serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse config {}", path.display()))?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create config directory {}", parent.display())
            })?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)
            .with_context(|| format!("failed to write config to {}", path.display()))?;
        Ok(())
    }

    pub fn effective_base_url(&self) -> String {
        std::env::var("FLOW_BASE_URL")
            .ok()
            .or_else(|| self.base_url.clone())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
    }

    pub fn effective_org(&self) -> Option<String> {
        std::env::var("FLOW_ORG")
            .ok()
            .or_else(|| self.org_alias.clone())
    }

    pub fn effective_project(&self) -> Option<String> {
        std::env::var("FLOW_PROJECT")
            .ok()
            .or_else(|| self.project_alias.clone())
    }
}

pub fn config_path() -> Result<PathBuf> {
    let path = dirs::config_dir()
        .ok_or_else(|| anyhow!("failed to determine config directory"))?
        .join("flow-cli")
        .join("config.json");
    Ok(path)
}
