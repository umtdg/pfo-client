use std::collections::HashSet;

use anyhow::{Context, Result};
use clap::Subcommand;
use pfo_core::output::{Table, TableArgs};
use pfo_core::sort::SortArguments;
use uuid::Uuid;

use crate::cli::fund::FundFilterArgs;
use crate::client::PfoClient;
use crate::fund::{FundInfo, FundInfoColumn, FundStats, FundStatsColumn};
use crate::portfolio::{
    Portfolio, PortfolioColumn, PortfolioFund, PortfolioFundColumn, PortfolioFundPrediction,
    PortfolioFundPredictionColumn, PortfolioFundPrice, PortfolioFundPriceColumn,
    PortfolioFundUpdate, PortfolioUpdate,
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
        name = "funds",
        visible_alias = "f",
        about = "Get a list of funds in portfolio"
    )]
    Funds {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[command(flatten)]
        output: TableArgs<PortfolioFundColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<PortfolioFundColumn>::value_parser,
            help = SortArguments::<PortfolioFundColumn>::get_help()
        )]
        sort: Option<SortArguments<PortfolioFundColumn>>,
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
        fund_filter: FundFilterArgs,

        #[command(flatten)]
        output: TableArgs<PortfolioFundPredictionColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<PortfolioFundPredictionColumn>::value_parser,
            help = SortArguments::<PortfolioFundPredictionColumn>::get_help()
        )]
        sort: Option<SortArguments<PortfolioFundPredictionColumn>>,
    },

    #[command(
        name = "info",
        visible_alias = "i",
        about = "Get fund information for funds in given portfolio"
    )]
    Info {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

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

    #[command(
        name = "stats",
        visible_alias = "s",
        about = "Get fund stats for funds in given portfolio"
    )]
    Stats {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[arg(
            short,
            long,
            help = "Forces server to update its internal fund stats by passing `force=true`"
        )]
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
            PortfolioCommand::Funds { id, output, sort } => {
                PortfolioFund::print_table(&client.get_portfolio_funds(id, sort).await?, output);
            }
            PortfolioCommand::Prices { id, output, sort } => {
                PortfolioFundPrice::print_table(
                    &client.get_portfolio_fund_prices(id, sort).await?,
                    output,
                );
            }
            PortfolioCommand::Predictions {
                id,
                budget,
                fund_filter,
                output,
                sort,
                ..
            } => {
                PortfolioFundPrediction::print_table(
                    &client
                        .get_portfolio_fund_predictions(id, budget, fund_filter, sort)
                        .await?,
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
            PortfolioCommand::Info { id, output, sort } => {
                FundInfo::print_table(&client.get_portfolio_fund_infos(id, sort).await?, output);
            }
            PortfolioCommand::Stats {
                id,
                force,
                output,
                sort,
            } => {
                FundStats::print_table(
                    &client.get_protfolio_fund_stats(id, sort, force).await?,
                    output,
                );
            }
        }

        Ok(())
    }
}
