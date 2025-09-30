use anyhow::Result;
use chrono::NaiveDate;
use clap::{Args, Subcommand};
use pfo_core::parse_naive_date;
use serde::Serialize;

use crate::client::Client;
use crate::fund::{
    FundInfo, FundInfoColumn, FundInfoSortBy, FundStats, FundStatsColumn, FundStatsSortBy,
};
use crate::output::{OutputArgs, OutputTable};

#[derive(Args, Serialize)]
pub struct FundFilterArgs {
    #[arg(short, long, value_parser = parse_naive_date)]
    pub date: Option<NaiveDate>,

    #[serde(rename = "fetchFrom")]
    #[arg(short, long, value_parser = parse_naive_date)]
    pub from: Option<NaiveDate>,

    #[arg(value_name = "FUND_CODES", value_delimiter = ',')]
    pub codes: Vec<String>,
}

#[derive(Subcommand)]
pub enum FundCommand {
    #[command(name = "get", visible_alias = "g", about = "Get fund(s)")]
    Get {
        #[command(flatten)]
        fund_filter: FundFilterArgs,

        #[command(flatten)]
        output: OutputArgs<FundInfoColumn, FundInfoSortBy>,
    },

    #[command(name = "stats", visible_alias = "s", about = "Get fund(s) statistics")]
    Stats {
        #[arg(value_name = "FUND_CODES", value_delimiter = ',')]
        codes: Vec<String>,

        #[arg(short, long)]
        force: bool,

        #[command(flatten)]
        output: OutputArgs<FundStatsColumn, FundStatsSortBy>,
    },
}

impl FundCommand {
    pub async fn handle(self, client: Client) -> Result<()> {
        match self {
            FundCommand::Get {
                fund_filter,
                output,
            } => {
                FundInfo::print_table(&client.get_funds(fund_filter, &output.sort).await?, output);
            }
            FundCommand::Stats {
                codes,
                force,
                output,
            } => {
                FundStats::print_table(
                    &client.get_fund_stats(codes, force, &output.sort).await?,
                    output,
                );
            }
        }

        Ok(())
    }
}
