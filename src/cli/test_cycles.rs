// src/cli/test_cycles.rs
use super::ResourceContextArgs;
use clap::{Args, Subcommand};

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
