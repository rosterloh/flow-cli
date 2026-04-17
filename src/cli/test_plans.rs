// src/cli/test_plans.rs
use clap::Subcommand;

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Patch(PatchCollectionArgs),
}
