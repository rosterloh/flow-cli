// src/cli/test_plans.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
    Get(TestPlanItemArgs),
    Patch(PatchCollectionArgs),
    Delete(TestPlanItemArgs),
    CreateCycle(TestPlanItemPayloadArgs),
    SetImportId(PatchCollectionArgs),
    LinkTestCase(PatchCollectionArgs),
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
