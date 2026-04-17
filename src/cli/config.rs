// src/cli/config.rs
use clap::Subcommand;

use super::ContextArgs;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Show,
    Path,
    SetContext(ContextArgs),
}
