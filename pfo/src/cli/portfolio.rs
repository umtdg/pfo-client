use std::collections::HashSet;

use anyhow::{Context, Result};
use clap::Subcommand;
use pfo_core::output::{Table, TableArgs};
use pfo_core::sort::SortArguments;
use uuid::Uuid;

use crate::cli::fund::FundFilterArgs;
use crate::client::Client;
use crate::fund::{FundInfo, FundInfoColumn, FundStats, FundStatsColumn};
use crate::portfolio::{
    FundToBuy, FundToBuyColumn, Portfolio, PortfolioColumn, PortfolioFundAdd, PortfolioUpdate,
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
        about = "Get how much to spend for each fund in a portfolio"
    )]
    Prices {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portoflio UUID")]
        id: Uuid,

        #[arg(short, long, help = "Budget to spend on funds")]
        budget: f32,

        #[command(flatten)]
        fund_filter: FundFilterArgs,

        #[command(flatten)]
        output: TableArgs<FundToBuyColumn>,

        #[arg(
            short,
            long,
            value_parser = SortArguments::<FundToBuyColumn>::value_parser,
            help = SortArguments::<FundToBuyColumn>::get_help()
        )]
        sort: Option<SortArguments<FundToBuyColumn>>,
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

        #[arg(short, long, help = "Forces server to update its internal fund stats by passing `force=true`")]
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

        #[arg(short, long, default_value_t = 50, help = "Weight of the added fund, higher means more preferred")]
        weight: u32,

        #[arg(long, default_value_t = 1, help = "Minimum number of amounts to buy the added fund")]
        min_amount: u32,
    },

    #[command(
        name = "remove",
        visible_alias = "rm",
        about = "Remove funds from a portfolio"
    )]
    Remove {
        #[arg(value_name = "PORTFOLIO_ID", help = "Portfolio UUID")]
        id: Uuid,

        #[arg(short, long, value_name = "FUND_CODE", help = "List of fund codes to remove")]
        codes: Vec<String>,
    },
}

impl PortfolioCommand {
    pub async fn handle(self, client: Client) -> Result<()> {
        match self {
            PortfolioCommand::List { output, .. } => {
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
                ..
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
            PortfolioCommand::Info { id, output, sort } => {
                FundInfo::print_table(&client.get_portfolio_fund_infos(id, &sort).await?, output);
            }
            PortfolioCommand::Stats {
                id,
                force,
                output,
                sort,
            } => {
                FundStats::print_table(
                    &client.get_protfolio_fund_stats(id, &sort, force).await?,
                    output,
                );
            }
        }

        Ok(())
    }
}
