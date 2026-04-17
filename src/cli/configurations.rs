// src/cli/configurations.rs
use super::{PatchCollectionArgs, ResourceContextArgs};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConfigurationCommands {
    List(ResourceContextArgs),
    Create(PatchCollectionArgs),
}
