// src/cli/systems.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    List(ListSystemsArgs),
    Create(CreateSystemArgs),
    Update(UpdateSystemArgs),
    Delete(SystemItemArgs),
    BulkUpdate(PatchCollectionArgs),
    ListDocuments(SystemItemArgs),
    LinkDocument(SystemLinkPayloadArgs),
    ListRequirements(SystemItemArgs),
    LinkRequirement(SystemLinkPayloadArgs),
    UnlinkRequirement(SystemUnlinkRequirementArgs),
    ListTestCases(SystemItemArgs),
    LinkTestCase(SystemLinkPayloadArgs),
    UnlinkTestCase(SystemUnlinkTestCaseArgs),
    ListTestPlans(SystemItemArgs),
    LinkTestPlan(SystemLinkPayloadArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ListSystemsArgs {
    #[command(flatten)]
    pub list: ListArgs,
    #[arg(long, help = "Show only top-level systems (no parent)")]
    pub top_level: bool,
}

#[derive(Args, Debug, Clone)]
pub struct SystemItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct SystemLinkPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct SystemUnlinkRequirementArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct SystemUnlinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub test_case_id: i64,
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
