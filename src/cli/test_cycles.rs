// src/cli/test_cycles.rs
use clap::{Args, Subcommand};
use super::ResourceContextArgs;

#[derive(Subcommand, Debug)]
pub enum TestCycleCommands {
    Get(TestCycleItemArgs),
    Delete(TestCycleItemArgs),
}

#[derive(Args, Debug)]
pub struct TestCycleItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}
