use clap::{Args, Parser, Subcommand, ValueEnum};
use reqwest::Method;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "flow-cli",
    about = "CLI for the Flow Engineering REST API",
    version,
    after_help = "Examples:\n  flow-cli auth exchange --refresh-token \"$FLOW_REFRESH_TOKEN\" --save\n  flow-cli config set-context --org my-org --project my-project\n  flow-cli orgs list\n  flow-cli projects list --org my-org\n  flow-cli requirements list --paged --limit 50\n  flow-cli raw GET /orgs\n"
)]
pub struct Cli {
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

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Show,
    Path,
    SetContext(ContextArgs),
}

#[derive(Subcommand, Debug)]
pub enum OrgsCommands {
    List,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    List(ResourceContextArgs),
    Create(CreateProjectArgs),
}

#[derive(Subcommand, Debug)]
pub enum RequirementCommands {
    List(ListArgs),
    Get(ItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(ItemArgs),
}

#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    List(ListArgs),
    Create(CreateSystemArgs),
    Update(UpdateSystemArgs),
    Delete(SystemItemArgs),
}

#[derive(Subcommand, Debug)]
pub enum TestCaseCommands {
    List(ListArgs),
    Get(TestCaseItemArgs),
    Create(CreateNamedItemsArgs),
    Patch(PatchCollectionArgs),
    Delete(TestCaseItemArgs),
}

#[derive(Subcommand, Debug)]
pub enum TestPlanCommands {
    List(ResourceContextArgs),
    Patch(PatchCollectionArgs),
}

#[derive(Subcommand, Debug)]
pub enum ValueCommands {
    List(ListValuesArgs),
    SetNumber(SetNumberValueArgs),
}

#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    ConvertHtml(ConvertHtmlArgs),
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

#[derive(Args, Debug, Clone)]
pub struct TestCaseItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug, Clone)]
pub struct SystemItemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct CreateProjectArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long)]
    pub name: String,
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

#[derive(Args, Debug)]
pub struct CreateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long, help = "User email, name, or id")]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long, help = "Must match ^[A-Z0-9_-]+$")]
    pub prefix: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateSystemArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub owner: Option<String>,
    #[arg(long)]
    pub parent_id: Option<String>,
    #[arg(long)]
    pub prefix: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct ListValuesArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long, help = "Use the numeric values endpoint")]
    pub numeric: bool,
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

#[derive(Args, Debug)]
pub struct ConvertHtmlArgs {
    #[arg(long = "html", required = true)]
    pub html: Vec<String>,
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
