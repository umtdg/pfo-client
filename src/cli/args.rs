use clap::{Parser, Subcommand, ArgAction};
use uuid::Uuid;

use crate::output::{FundToBuyColumn, PortfolioColumn};

#[derive(Parser)]
#[command(name = "portfolio-cli")]
#[clap(disable_help_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'H', long, global = true)]
    pub host: Option<String>,

    #[arg(short, long, global = true)]
    pub port: Option<u16>,

    #[arg(short, long, global = true)]
    pub debug: bool,

    #[arg(short, long, global = true)]
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
    // Fund(FundCommand),
    // Login(LoginCommand),
}

#[derive(Subcommand)]
pub enum PortfolioCommand {
    #[command(name = "list", visible_alias = "ls", about = "List all portfolios")]
    List {
        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<PortfolioColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },

    #[command(name = "get", visible_alias = "g", about = "Get single portfolio")]
    Get {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<PortfolioColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },

    #[command(name = "prices", visible_alias = "p", about = "Get how much to spend for each fund in a portfolio")]
    Prices {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long)]
        budget: f32,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<FundToBuyColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },

    #[command(name = "add", visible_alias = "a", about = "Add funds to a portfolio")]
    Add {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long, value_name = "FUND_CODE")]
        code: String,

        #[arg(short, long, default_value_t = 50)]
        weight: u32,

        #[arg(long, default_value_t = 1)]
        min_amount: u32,
    },

    #[command(name = "remove", visible_alias = "rm", about = "Remove funds from a portfolio")]
    Remove {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long, value_name = "FUND_CODE")]
        codes: Vec<String>,
    },
}
