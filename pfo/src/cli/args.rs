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

    #[arg(short = 'H', long, global = true, help = "Server hostname/IP")]
    pub host: Option<String>,

    #[arg(short, long, global = true, help = "Server port")]
    pub port: Option<u16>,
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

                generate(
                    generator,
                    &mut cmd,
                    bin_name,
                    &mut io::stdout(),
                );

                Ok(())
            }
        }
    }
}
