// src/cli/orgs.rs
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum OrgsCommands {
    List,
}
