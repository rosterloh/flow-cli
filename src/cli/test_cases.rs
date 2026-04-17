// src/cli/test_cases.rs
use clap::{Args, Subcommand};

use super::{CreateNamedItemsArgs, JsonPayloadArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestCaseCommands {
    List(ListArgs),
    Get(TestCaseItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(TestCaseItemArgs),
    SetSteps(TestCaseItemPayloadArgs),
    SetImportId(PatchCollectionArgs),
    UploadFile(TestCaseUploadFileArgs),
    LinkJira(TestCaseItemPayloadArgs),
    UnlinkJira(TestCaseUnlinkJiraArgs),
    ListRequirements(TestCaseItemArgs),
    GetCustomFields(ResourceContextArgs),
    PatchCustomFields(PatchCollectionArgs),
    RenameCustomFieldOption(PatchCollectionArgs),
    AddConfiguration(PatchCollectionArgs),
    RemoveConfiguration(PatchCollectionArgs),
    SetStages(PatchCollectionArgs),
    CreateTestRun(TestCaseItemPayloadArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestCaseItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestCaseItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct TestCaseUploadFileArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub file_id: String,
}

#[derive(Args, Debug)]
pub struct TestCaseUnlinkJiraArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub jira_issue_id: String,
}
