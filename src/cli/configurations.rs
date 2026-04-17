// src/cli/configurations.rs
use clap::Subcommand;
use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum ConfigurationCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
}
