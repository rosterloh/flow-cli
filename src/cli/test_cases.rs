// src/cli/test_cases.rs
use clap::{Args, Subcommand};

use super::{CreateNamedItemsArgs, ListArgs, PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestCaseCommands {
    List(ListArgs),
    Get(TestCaseItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(TestCaseItemArgs),
}

#[derive(Args, Debug, Clone)]
pub struct TestCaseItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
