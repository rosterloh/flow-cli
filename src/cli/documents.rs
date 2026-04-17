// src/cli/documents.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum DocumentCommands {
    List(ResourceContextArgs),
    Get(DocumentItemArgs),
    Create(PatchCollectionArgs),
    Patch(PatchCollectionArgs),
    Delete(DocumentItemArgs),
    SetImportId(PatchCollectionArgs),
}

#[derive(Args, Debug)]
pub struct DocumentItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
