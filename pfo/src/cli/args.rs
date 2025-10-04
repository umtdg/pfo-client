use clap::{ArgAction, Parser, Subcommand};

use crate::cli::Command;
use crate::cli::fund::FundCommand;
use crate::cli::portfolio::PortfolioCommand;

#[derive(Parser)]
#[command(name = "pfo")]
#[clap(disable_help_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'H', long, global = true)]
    pub host: Option<String>,

    #[arg(short, long, global = true)]
    pub port: Option<u16>,

    #[arg(long, global = true)]
    pub config: Option<String>,

    #[arg(short, long, global = true, action = ArgAction::HelpLong)]
    pub help: Option<bool>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "portfolio", visible_alias = "p", about = "Portfolio actions")]
    Portfolio {
        #[command(subcommand)]
        command: PortfolioCommand,
    },

    #[command(name = "fund", visible_alias = "f", about = "Fund actions")]
    Fund {
        #[command(subcommand)]
        command: FundCommand,
    },
}

impl Command for Commands {
    async fn handle(self, client: crate::client::Client) -> anyhow::Result<()> {
        match self {
            Commands::Portfolio { command } => command.handle(client).await,
            Commands::Fund { command } => command.handle(client).await,
        }
    }
}
