use anyhow::Result;
use chrono::NaiveDate;
use clap::{Args, Subcommand};
use pfo_core::sort::SortArguments;
use serde::Serialize;

use crate::fund::{FundInfo, FundInfoColumn, FundStats, FundStatsColumn};
use pfo_core::output::{Table, TableArgs};
use pfo_core::parse_naive_date;

use crate::client::PfoClient;

#[derive(Args, Serialize)]
pub struct FundFilterArgs {
    #[arg(
        short,
        long,
        value_parser = parse_naive_date,
        help = "Filter output to given date, otherwise no date is sent in query and server decides",
    )]
    pub date: Option<NaiveDate>,

    #[serde(rename = "fetchFrom")]
    #[arg(
        short,
        long,
        value_parser = parse_naive_date,
        help = "Date from which server should update its information. Fetch range is <fetchFrom> - <date>",
    )]
    pub from: Option<NaiveDate>,

    #[arg(value_name = "FUND_CODES", value_delimiter = ',', help = "List of fund codes to")]
    pub codes: Vec<String>,
}

#[derive(Subcommand)]
pub enum FundCommand {
    #[command(name = "get", visible_alias = "g", about = "Get fund(s)")]
    Get {
        #[command(flatten)]
        fund_filter: FundFilterArgs,

        #[command(flatten)]
        output: TableArgs<FundInfoColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<FundInfoColumn>::value_parser,
            help = SortArguments::<FundInfoColumn>::get_help()
        )]
        sort: Option<SortArguments<FundInfoColumn>>,
    },

    #[command(name = "stats", visible_alias = "s", about = "Get fund(s) statistics")]
    Stats {
        #[arg(value_name = "FUND_CODES", value_delimiter = ',', help = "List of fund codes to get fund statistics")]
        codes: Vec<String>,

        #[arg(short, long, help = "Forces server to update its internal fund stats by passing `force=true` to GET query")]
        force: bool,

        #[command(flatten)]
        output: TableArgs<FundStatsColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<FundStatsColumn>::value_parser,
            help = SortArguments::<FundStatsColumn>::get_help()
        )]
        sort: Option<SortArguments<FundStatsColumn>>,
    },
}

impl FundCommand {
    pub async fn handle(self, client: PfoClient) -> Result<()> {
        match self {
            FundCommand::Get {
                fund_filter,
                output,
                sort,
            } => {
                FundInfo::print_table(&client.get_funds(fund_filter, sort).await?, output);
            }
            FundCommand::Stats {
                codes,
                force,
                output,
                sort,
            } => {
                FundStats::print_table(&client.get_fund_stats(codes, force, sort).await?, output);
            }
        }

        Ok(())
    }
}
