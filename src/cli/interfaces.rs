// src/cli/interfaces.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum InterfaceCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
    Patch(PatchCollectionArgs),
    Delete(InterfaceItemArgs),
}

#[derive(Args, Debug)]
pub struct InterfaceItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
