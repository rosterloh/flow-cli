// src/cli/mod.rs
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use reqwest::Method;

pub mod auth;
pub mod config;
pub mod orgs;
pub mod projects;
pub mod requirements;
pub mod systems;
pub mod test_cases;
pub mod test_plans;
pub mod values;
pub mod util;

pub use auth::{AuthCommands, ExchangeAuthArgs, SetBasicArgs, SetBearerArgs};
pub use config::ConfigCommands;
pub use orgs::OrgsCommands;
pub use projects::{CreateProjectArgs, ProjectCommands};
pub use requirements::{
    ListRequirementsArgs, RequirementCommands, RequirementFileArgs, RequirementImageArgs,
    RequirementJiraArgs, RequirementLinkArgs, RequirementLinkTestCaseArgs, RequirementScope,
    RequirementUnlinkArgs, RequirementUnlinkCrossProjectArgs, RequirementUnlinkJiraArgs,
};
pub use systems::{
    CreateSystemArgs, SystemCommands, SystemItemArgs, SystemLinkPayloadArgs,
    SystemUnlinkRequirementArgs, SystemUnlinkTestCaseArgs, UpdateSystemArgs,
};
pub use test_cases::{
    TestCaseCommands, TestCaseItemArgs, TestCaseItemPayloadArgs,
    TestCaseUnlinkJiraArgs, TestCaseUploadFileArgs,
};
pub use test_plans::TestPlanCommands;
pub use values::{ListValuesArgs, SetNumberValueArgs, ValueCommands};
pub use util::{ConvertHtmlArgs, UtilCommands};

#[derive(Parser, Debug)]
#[command(
    name = "flow-cli",
    about = "CLI for the Flow Engineering REST API",
    version,
    after_help = "Examples:\n  flow-cli auth exchange --refresh-token \"$FLOW_REFRESH_TOKEN\" --save\n  flow-cli config set-context --org my-org --project my-project\n  flow-cli orgs list\n  flow-cli projects list --org my-org\n  flow-cli requirements list --paged --limit 50\n  flow-cli raw GET /orgs\n"
)]
pub struct Cli {
    #[arg(long, global = true, default_value = "json", value_enum)]
    pub output: crate::output::OutputFormat,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    Orgs {
        #[command(subcommand)]
        command: OrgsCommands,
    },
    Projects {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    Requirements {
        #[command(subcommand)]
        command: RequirementCommands,
    },
    Systems {
        #[command(subcommand)]
        command: SystemCommands,
    },
    TestCases {
        #[command(subcommand)]
        command: TestCaseCommands,
    },
    TestPlans {
        #[command(subcommand)]
        command: TestPlanCommands,
    },
    Values {
        #[command(subcommand)]
        command: ValueCommands,
    },
    Util {
        #[command(subcommand)]
        command: UtilCommands,
    },
    Raw(RawCommand),
}

#[derive(Args, Debug, Clone)]
pub struct ResourceContextArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long, env = "FLOW_PROJECT")]
    pub project: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ContextArgs {
    #[arg(long)]
    pub org: Option<String>,
    #[arg(long)]
    pub project: Option<String>,
    #[arg(long)]
    pub base_url: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the paginated endpoint when available")]
    pub paged: bool,
    #[arg(long)]
    pub after: Option<String>,
    #[arg(long)]
    pub limit: Option<u32>,
}

#[derive(Args, Debug, Clone)]
pub struct ItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct CreateNamedItemsArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long = "name", required = true)]
    pub names: Vec<String>,
    #[arg(long)]
    pub description: Option<String>,
}

#[derive(Args, Debug)]
pub struct PatchCollectionArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug, Default)]
pub struct JsonPayloadArgs {
    #[arg(long, conflicts_with = "body_file")]
    pub json: Option<String>,
    #[arg(long, value_name = "PATH", conflicts_with = "json")]
    pub body_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct RawCommand {
    pub method: HttpMethod,
    pub path: String,
    #[arg(long = "query", value_name = "KEY=VALUE")]
    pub query: Vec<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_method(self) -> Method {
        match self {
            Self::Get => Method::GET,
            Self::Post => Method::POST,
            Self::Put => Method::PUT,
            Self::Patch => Method::PATCH,
            Self::Delete => Method::DELETE,
        }
    }
}
