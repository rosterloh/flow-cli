// src/cli/projects.rs
use clap::{Args, Subcommand};

use super::ResourceContextArgs;

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    List(ResourceContextArgs),
    Create(CreateProjectArgs),
}

#[derive(Args, Debug)]
pub struct CreateProjectArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long)]
    pub name: String,
}
