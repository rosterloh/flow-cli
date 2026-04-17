// src/cli/members.rs
use clap::{Args, Subcommand};

use super::{JsonPayloadArgs, ResourceContextArgs};

#[derive(Subcommand, Debug)]
pub enum MemberCommands {
    ListOrg(OrgMemberArgs),
    AddOrg(OrgAddMemberArgs),
    RemoveOrg(OrgRemoveMemberArgs),
    ListProject(ProjectMemberArgs),
    AddProject(ProjectAddMemberArgs),
    RemoveProject(ProjectRemoveMemberArgs),
}

#[derive(Args, Debug)]
pub struct OrgMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
}

#[derive(Args, Debug)]
pub struct OrgAddMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct OrgRemoveMemberArgs {
    #[arg(long, env = "FLOW_ORG")]
    pub org: Option<String>,
    #[arg(long)]
    pub email: String,
}

#[derive(Args, Debug)]
pub struct ProjectMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
}

#[derive(Args, Debug)]
pub struct ProjectAddMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[command(flatten)]
    pub payload: JsonPayloadArgs,
}

#[derive(Args, Debug)]
pub struct ProjectRemoveMemberArgs {
    #[command(flatten)]
    pub context: ResourceContextArgs,
    #[arg(long)]
    pub email: String,
}
