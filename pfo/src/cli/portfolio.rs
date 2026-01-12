use std::collections::HashSet;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::Subcommand;
use pfo_core::output::{Table, TableArgs};
use pfo_core::parse_naive_date;
use pfo_core::sort::SortArguments;
use uuid::Uuid;

use crate::client::PfoClient;
use crate::fund::{FundPriceStats, FundPriceStatsColumn};
use crate::portfolio::{
    Portfolio, PortfolioColumn, PortfolioFundPrediction, PortfolioFundPredictionColumn,
    PortfolioFundPrice, PortfolioFundPriceColumn, PortfolioFundUpdate, PortfolioUpdate,
};

#[derive(Subcommand)]
pub enum PortfolioCommand {
    #[command(name = "list", visible_alias = "ls", about = "List all portfolios")]
    List {
        #[command(flatten)]
        output: TableArgs<PortfolioColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<PortfolioColumn>::value_parser,
            help = SortArguments::<PortfolioColumn>::get_help()
        )]
        sort: Option<SortArguments<PortfolioColumn>>,
    },

    #[command(name = "get", visible_alias = "g", about = "Get single portfolio")]
    Get {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[command(flatten)]
        output: TableArgs<PortfolioColumn>,
    },

    #[command(
        name = "prices",
        visible_alias = "p",
        about = "Get a list of fund prices in portfolio"
    )]
    Prices {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[command(flatten)]
        output: TableArgs<PortfolioFundPriceColumn>,

        #[arg(
            short,
            long,
            value_parser = parse_naive_date,
            help = "Get fund prices only for given date. Latest date is used by server if omitted"
        )]
        date: Option<NaiveDate>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<PortfolioFundPriceColumn>::value_parser,
            help = SortArguments::<PortfolioFundPriceColumn>::get_help()
        )]
        sort: Option<SortArguments<PortfolioFundPriceColumn>>,
    },

    #[command(
        name = "predictions",
        visible_alias = "P",
        visible_alias = "pred",
        about = "Get how much to spend for each fund in a portfolio"
    )]
    Predictions {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portoflio UUID")]
        id: Uuid,

        #[arg(short, long, help = "Budget to spend on funds")]
        budget: f32,

        #[command(flatten)]
        output: TableArgs<PortfolioFundPredictionColumn>,
    },

    #[command(
        name = "price-stats",
        visible_alias = "ps",
        about = "Get fund price stats for funds in given portfolio"
    )]
    PriceStats {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[command(flatten)]
        output: TableArgs<FundPriceStatsColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<FundPriceStatsColumn>::value_parser,
            help = SortArguments::<FundPriceStatsColumn>::get_help()
        )]
        sort: Option<SortArguments<FundPriceStatsColumn>>,
    },

    #[command(name = "add", visible_alias = "a", about = "Add funds to a portfolio")]
    Add {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[arg(short, long, value_name = "FUND_CODE", help = "Fund code to add")]
        code: String,

        #[arg(
            short,
            long,
            help = "Weight of the added fund, higher means more preferred"
        )]
        weight: Option<u32>,

        #[arg(long, help = "Minimum number of amounts to buy the added fund")]
        min_amount: Option<u32>,

        #[arg(short, long, help = "Owned amount")]
        owned_amount: Option<u32>,

        #[arg(
            short,
            long,
            help = "Total money spent for buying owned_amount many units"
        )]
        total_money_spent: Option<f64>,
    },

    #[command(
        name = "remove",
        visible_alias = "rm",
        about = "Remove funds from a portfolio"
    )]
    Remove {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[arg(
            short,
            long,
            value_name = "FUND_CODE",
            help = "List of fund codes to remove"
        )]
        codes: Vec<String>,
    },
}

impl PortfolioCommand {
    pub async fn handle(self, client: PfoClient) -> Result<()> {
        match self {
            PortfolioCommand::List { output, .. } => {
                Portfolio::print_table(&client.list_portfolios().await?, output);
            }
            PortfolioCommand::Get { id, output } => {
                Portfolio::print_table(&[client.get_portfolio(id).await?], output);
            }
            PortfolioCommand::Prices {
                id,
                output,
                date,
                sort,
            } => {
                PortfolioFundPrice::print_table(
                    &client.get_portfolio_fund_prices(id, date, sort).await?,
                    output,
                );
            }
            PortfolioCommand::Predictions { id, budget, output } => {
                PortfolioFundPrediction::print_table(
                    &client.get_portfolio_fund_predictions(id, budget).await?,
                    output,
                );
            }
            PortfolioCommand::Add {
                id,
                code,
                weight,
                min_amount,
                owned_amount,
                total_money_spent,
            } => {
                let update = PortfolioUpdate {
                    add_codes: {
                        let mut set = HashSet::new();
                        set.insert(PortfolioFundUpdate {
                            fund_code: code,
                            weight,
                            min_amount,
                            owned_amount,
                            total_money_spent,
                        });
                        set
                    },
                    remove_codes: HashSet::new(),
                };

                client
                    .update_portfolio(id, update)
                    .await
                    .context("Failed to add fund to portfolio")?;

                println!("Successfully added fund");
            }
            PortfolioCommand::Remove {
                id,
                codes: fund_codes,
            } => {
                let update = PortfolioUpdate {
                    add_codes: HashSet::new(),
                    remove_codes: fund_codes.into_iter().collect(),
                };

                client
                    .update_portfolio(id, update)
                    .await
                    .context("Failed to remove funds from portfolio")?;

                println!("Successfully removed funds");
            }
            PortfolioCommand::PriceStats { id, output, sort } => {
                FundPriceStats::print_table(
                    &client.get_portfolio_fund_price_stats(id, sort).await?,
                    output,
                );
            }
        }

        Ok(())
    }
}
