use anyhow::Result;
use chrono::NaiveDate;
use clap::Subcommand;
use pfo_core::parse_naive_date;

use crate::cli::SortArguments;
use crate::client::Client;
use crate::fund::{
    FundInfo, FundInfoColumn, FundInfoSortBy, FundStats, FundStatsColumn, FundStatsSortBy,
};
use crate::output::OutputTable;

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

        #[arg(
            short,
            long,
            default_value = "code asc",
            value_parser = SortArguments::<FundInfoSortBy>::value_parser,
            help = SortArguments::<FundInfoSortBy>::get_help()
        )]
        sort: SortArguments<FundInfoSortBy>,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<FundInfoColumn>>,

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
            default_value = "five-yearly-return desc",
            value_parser = SortArguments::<FundStatsSortBy>::value_parser,
            help = SortArguments::<FundStatsSortBy>::get_help())
        ]
        sort: SortArguments<FundStatsSortBy>,

        #[arg(short, long, value_delimiter = ',')]
        output: Option<Vec<FundStatsColumn>>,

        #[arg(long)]
        no_headers: bool,

        #[arg(short, long)]
        wide: bool,
    },
}

impl FundCommand {
    pub async fn handle(self, client: Client) -> Result<()> {
        match self {
            FundCommand::Get {
                codes,
                date,
                from,
                sort,
                output,
                no_headers,
                wide,
            } => {
                let fund_infos = client.get_funds(codes, date, from, sort).await?;
                let columns = output.unwrap_or(vec![
                    FundInfoColumn::Code,
                    FundInfoColumn::Title,
                    FundInfoColumn::Date,
                    FundInfoColumn::Price,
                    FundInfoColumn::TotalValue,
                ]);
                let headers = !no_headers;

                FundInfo::print_table(&fund_infos, &columns, headers, wide);
            }
            FundCommand::Stats {
                codes,
                force,
                sort,
                output,
                no_headers,
                wide,
            } => {
                let fund_stats = client.get_fund_stats(codes, force, sort).await?;
                let columns = output.unwrap_or(vec![
                    FundStatsColumn::Code,
                    FundStatsColumn::LastPrice,
                    FundStatsColumn::TotalValue,
                    FundStatsColumn::Yearly,
                    FundStatsColumn::ThreeYearly,
                    FundStatsColumn::FiveYearly,
                ]);
                let headers = !no_headers;

                FundStats::print_table(&fund_stats, &columns, headers, wide);
            }
        }

        Ok(())
    }
}
