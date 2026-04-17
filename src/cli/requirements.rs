// src/cli/requirements.rs
use clap::Subcommand;

use super::{CreateNamedItemsArgs, ItemArgs, ListArgs, PatchCollectionArgs};

#[derive(Subcommand, Debug)]
pub enum RequirementCommands {
    List(ListArgs),
    Get(ItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(ItemArgs),
}
