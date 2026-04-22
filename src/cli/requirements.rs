// src/cli/requirements.rs
use clap::{Args, Subcommand, ValueEnum};

use super::{
    CreateNamedItemsArgs, ItemArgs, JsonPayloadArgs, ListArgs, PatchCollectionArgs,
    ResourceContextArgs,
};

#[derive(Subcommand, Debug)]
pub enum RequirementCommands {
    List(ListRequirementsArgs),
    Search(RequirementSearchArgs),
    Get(ItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(RequirementPatchArgs),
    Delete(ItemArgs),
    Filter(PatchCollectionArgs),
    SetStage(PatchCollectionArgs),
    SetImportId(PatchCollectionArgs),
    SetValue(PatchCollectionArgs),
    ListTestCases(ItemArgs),
    ListTestPlans(ItemArgs),
    UploadFile(RequirementFileArgs),
    UploadImage(RequirementImageArgs),
    LinkJira(RequirementJiraArgs),
    UnlinkJira(RequirementUnlinkJiraArgs),
    Link(RequirementLinkArgs),
    Unlink(RequirementUnlinkArgs),
    UnlinkCrossProject(RequirementUnlinkCrossProjectArgs),
    LinkTestCase(RequirementLinkTestCaseArgs),
    LinkTestCaseCrossProject(RequirementLinkTestCaseArgs),
    GetCustomFields(ResourceContextArgs),
    PatchCustomFields(PatchCollectionArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
    AddConfiguration(PatchCollectionArgs),
    RemoveConfiguration(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ListRequirementsArgs {
    #[command(flatten)]
    pub list: ListArgs,
    #[arg(long, value_enum)]
    pub scope: Option<RequirementScope>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RequirementScope {
    Org,
    Project,
    WithoutSystem,
}

#[derive(Args, Debug)]
pub struct RequirementFileArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementImageArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub file_id: String,
}

#[derive(Args, Debug)]
pub struct RequirementJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub jira_issue_id: String,
}

#[derive(Args, Debug)]
pub struct RequirementLinkArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[arg(long)]
    pub linked_requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct RequirementUnlinkCrossProjectArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub link_type: String,
    #[arg(long)]
    pub linked_project: String,
    #[arg(long)]
    pub linked_requirement_id: i64,
}

#[derive(Args, Debug)]
pub struct RequirementLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct RequirementSearchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(help = "Search term — case-insensitive, matches anywhere in the requirement name")]
    pub term: String,
}

#[derive(Args, Debug)]
pub struct RequirementPatchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub name: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub owner: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
