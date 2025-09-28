use chrono::NaiveDate;
use clap::{ArgAction, Parser, Subcommand};
use uuid::Uuid;

use crate::{
    cli::sort::{SortArguments, SortByFundInfo, SortByFundStats},
    output::{FundInformationColumn, FundStatsColumn, FundToBuyColumn, PortfolioColumn},
};

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

    #[command(name = "fund", visible_alias = "f", about = "Fund actions")]
    Fund {
        #[command(subcommand)]
        command: FundCommand,
    },
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

    #[command(
        name = "prices",
        visible_alias = "p",
        about = "Get how much to spend for each fund in a portfolio"
    )]
    Prices {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long)]
        budget: f32,

        #[arg(short, long, value_parser = parse_naive_date)]
        date: Option<NaiveDate>,

        #[arg(short, long, value_parser = parse_naive_date)]
        from: Option<NaiveDate>,

        #[arg(short, long, value_delimiter = ',')]
        codes: Vec<String>,

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

    #[command(
        name = "remove",
        visible_alias = "rm",
        about = "Remove funds from a portfolio"
    )]
    Remove {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long, value_name = "FUND_CODE")]
        codes: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum FundCommand {
    #[command(name = "get", visible_alias = "g", about = "Get fund(s)")]
    Get {
        #[arg(value_name = "FUND_CODES", value_delimiter = ',')]
        codes: Vec<String>,

        #[arg(short, long, value_parser = parse_naive_date)]
        date: Option<NaiveDate>,

        #[arg(short, long, value_parser = parse_naive_date)]
        from: Option<NaiveDate>,

        #[arg(short, long, value_parser = SortArguments::<SortByFundInfo>::value_parser)]
        sort: SortArguments<SortByFundInfo>,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<FundInformationColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },

    #[command(name = "stats", visible_alias = "s", about = "Get fund(s) statistics")]
    Stats {
        #[arg(value_name = "FUND_CODES", value_delimiter = ',')]
        codes: Vec<String>,

        #[arg(short, long)]
        force: bool,

        #[arg(
            short, long,
            default_value = "fiveYearlyReturn desc",
            value_parser = SortArguments::<SortByFundStats>::value_parser,
            help = SortArguments::<SortByFundStats>::help())
        ]
        sort: SortArguments<SortByFundStats>,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<FundStatsColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },
}

pub fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%m.%d.%Y")
}
