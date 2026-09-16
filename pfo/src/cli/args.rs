use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

use crate::cli::fund::FundCommand;
use crate::cli::portfolio::PortfolioCommand;

#[derive(Parser)]
#[command(name = "pfo")]
pub struct Args {
    #[command(subcommand, help = "Subcommand")]
    pub command: Commands,

    /// Deprecated: use [`Args::base_url`]
    #[arg(
        short = 'H',
        long,
        global = true,
        hide = true,
        help = "Server hostname/IP (Deprecated, use --base-url)"
    )]
    pub host: Option<String>,

    /// Deprecated: use [`Args::base_url`]
    #[arg(
        short,
        long,
        global = true,
        hide = true,
        help = "Server port (Deprecated, use --base-url)"
    )]
    pub port: Option<u16>,

    #[arg(
        short = 'u',
        long,
        global = true,
        default_value = "https://pfo.umtdg.com",
        conflicts_with_all = ["host", "port"],
        help = "Base URL for the server where the requests will be made"
    )]
    pub base_url: String,
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

    #[command(
        name = "completions",
        visible_alias = "comp",
        about = "Print shell completions"
    )]
    Completions {
        #[arg(short, long)]
        generator: Shell,
    },
}

impl Commands {
    pub async fn handle(self, client: crate::client::PfoClient) -> anyhow::Result<()> {
        match self {
            Commands::Portfolio { command } => command.handle(client).await,
            Commands::Fund { command } => command.handle(client).await,
            Commands::Completions { generator } => {
                let mut cmd = Args::command();
                let bin_name = cmd.get_name().to_string();
                eprintln!("Generating completion for {generator:?}");

                generate(generator, &mut cmd, bin_name, &mut io::stdout());

                Ok(())
            }
        }
    }
}
