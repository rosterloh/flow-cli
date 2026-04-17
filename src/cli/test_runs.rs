// src/cli/test_runs.rs
use super::{JsonPayloadArgs, ResourceContextArgs};
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum TestRunCommands {
    Get(TestRunItemArgs),
    Patch(TestRunItemPayloadArgs),
    Delete(TestRunItemArgs),
    SetSteps(TestRunItemPayloadArgs),
}

#[derive(Args, Debug)]
pub struct TestRunItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub cycle_id: i64,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct TestRunItemPayloadArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub cycle_id: i64,
    #[arg(long)]
    pub id: i64,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}
