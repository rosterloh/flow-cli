// src/cli/test_plans.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
    Get(TestPlanItemArgs),
    Patch(TestPlanPatchArgs),
    Delete(TestPlanItemArgs),
    CreateCycle(TestPlanItemPayloadArgs),
    SetImportId(PatchCollectionArgs),
    LinkTestCase(TestPlanLinkTestCaseArgs),
    LinkTestCaseCrossProject(PatchCollectionArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestPlanItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestPlanItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct TestPlanPatchArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub name: Option<String>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub description: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct TestPlanLinkTestCaseArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_plan_id: Option<i64>,
    #[arg(long, conflicts_with_all = ["json", "body_file"])]
    pub test_case_id: Option<i64>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
