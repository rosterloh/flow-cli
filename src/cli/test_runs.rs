// src/cli/test_runs.rs
use clap::{Args, Subcommand};
use super::{JsonPayloadArgs, ResourceContextArgs};

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
