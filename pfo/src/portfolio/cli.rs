use std::collections::HashSet;

use anyhow::{Context, Result};
use clap::Subcommand;
use uuid::Uuid;

use crate::client::Client;
use crate::fund::{
    FundFilterArgs, FundInfo, FundInfoColumn, FundInfoSortBy, FundStats, FundStatsColumn,
    FundStatsSortBy,
};
use crate::output::{OutputArgs, OutputTable};
use crate::portfolio::{
    FundToBuy, FundToBuyColumn, FundToBuySortBy, Portfolio, PortfolioColumn, PortfolioFundAdd,
    PortfolioSortBy, PortfolioUpdate,
};

#[derive(Subcommand)]
pub enum PortfolioCommand {
    #[command(name = "list", visible_alias = "ls", about = "List all portfolios")]
    List {
        #[command(flatten)]
        output: OutputArgs<PortfolioColumn, PortfolioSortBy>,
    },

    #[command(name = "get", visible_alias = "g", about = "Get single portfolio")]
    Get {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[command(flatten)]
        output: OutputArgs<PortfolioColumn, PortfolioSortBy>,
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

        #[command(flatten)]
        fund_filter: FundFilterArgs,

        #[command(flatten)]
        output: OutputArgs<FundToBuyColumn, FundToBuySortBy>,
    },

    #[command(
        name = "info",
        visible_alias = "i",
        about = "Get fund information for funds in given portfolio"
    )]
    Info {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[command(flatten)]
        output: OutputArgs<FundInfoColumn, FundInfoSortBy>,
    },

    #[command(
        name = "stats",
        visible_alias = "s",
        about = "Get fund stats for funds in given portfolio"
    )]
    Stats {
        #[arg(value_name = "PORTFOLIO_ID")]
        id: Uuid,

        #[arg(short, long)]
        force: bool,

        #[command(flatten)]
        output: OutputArgs<FundStatsColumn, FundStatsSortBy>,
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

impl PortfolioCommand {
    pub async fn handle(self, client: Client) -> Result<()> {
        match self {
            PortfolioCommand::List { output } => {
                Portfolio::print_table(&client.list_portfolios().await?, output);
            }
            PortfolioCommand::Get { id, output } => {
                Portfolio::print_table(&vec![client.get_portfolio(id).await?], output);
            }
            PortfolioCommand::Prices {
                id,
                budget,
                fund_filter,
                output,
            } => {
                FundToBuy::print_table(
                    &client.get_portfolio_prices(id, budget, fund_filter).await?,
                    output,
                );
            }
            PortfolioCommand::Add {
                id,
                code,
                weight,
                min_amount,
            } => {
                let update = PortfolioUpdate {
                    add_codes: {
                        let mut set = HashSet::new();
                        set.insert(PortfolioFundAdd {
                            fund_code: code,
                            weight,
                            min_amount,
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
            PortfolioCommand::Info { id, output } => {
                FundInfo::print_table(
                    &client.get_portfolio_fund_infos(id, &output.sort).await?,
                    output,
                );
            }
            PortfolioCommand::Stats { id, force, output } => {
                FundStats::print_table(
                    &client
                        .get_protfolio_fund_stats(id, &output.sort, force)
                        .await?,
                    output,
                );
            }
        }

        Ok(())
    }
}
