// src/cli/auth.rs
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    Exchange(ExchangeAuthArgs),
    SetBearer(SetBearerArgs),
    SetBasic(SetBasicArgs),
    Status,
    Clear {
        #[arg(long, help = "Also clear the saved refresh token")]
        all: bool,
    },
}

#[derive(Args, Debug)]
pub struct ExchangeAuthArgs {
    #[arg(long, env = "FLOW_REFRESH_TOKEN")]
    pub refresh_token: Option<String>,
    #[arg(
        long,
        help = "Persist the refresh token as well as the new access token"
    )]
    pub save_refresh_token: bool,
    #[arg(long, help = "Persist the new access token")]
    pub save: bool,
}

#[derive(Args, Debug)]
pub struct SetBearerArgs {
    #[arg(long, env = "FLOW_ACCESS_TOKEN")]
    pub access_token: String,
    #[arg(long, help = "Persist the access token to the local config")]
    pub save: bool,
}

#[derive(Args, Debug)]
pub struct SetBasicArgs {
    #[arg(long, env = "FLOW_USERNAME")]
    pub username: String,
    #[arg(long, env = "FLOW_PASSWORD")]
    pub password: String,
    #[arg(long, help = "Persist the basic auth credentials to the local config")]
    pub save: bool,
}
