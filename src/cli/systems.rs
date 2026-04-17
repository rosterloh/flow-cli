// src/cli/systems.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ListArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    List(ListArgs),
    Create(CreateSystemArgs),
    Update(UpdateSystemArgs),
    Delete(SystemItemArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SystemItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct CreateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long, help = "User email, name, or id")]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long, help = "Must match ^[A-Z0-9_-]+$")]
    pub prefix: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long)]
    pub prefix: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
