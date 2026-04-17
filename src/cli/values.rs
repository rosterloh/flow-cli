// src/cli/values.rs
use clap::{Args, Subcommand};

use super::{PatchCollectionArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum ValueCommands {
    List(ListValuesArgs),
    Get(ValueItemArgs),
    SetNumber(SetNumberValueArgs),
    SetImportId(PatchCollectionArgs),
}

#[derive(Args, Debug)]
pub struct ListValuesArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the numeric values endpoint")]
    pub numeric: bool,
}

#[derive(Args, Debug)]
pub struct ValueItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct SetNumberValueArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
    #[arg(long)]
    pub value: f64,
}
