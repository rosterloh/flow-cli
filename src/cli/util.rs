// src/cli/util.rs
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    ConvertHtml(ConvertHtmlArgs),
}

#[derive(Args, Debug)]
pub struct ConvertHtmlArgs {
    #[arg(long = "html", required = true)]
    pub html: Vec<String>,
}
