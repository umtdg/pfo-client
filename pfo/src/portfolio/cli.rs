use std::collections::HashSet;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::Subcommand;
use pfo_core::{parse_naive_date, turkish_collate};
use uuid::Uuid;

use crate::client::Client;
use crate::output::OutputTable;
use crate::portfolio::{
    FundToBuy, FundToBuyColumn, Portfolio, PortfolioColumn, PortfolioFundAdd, PortfolioUpdate,
};

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

impl PortfolioCommand {
    pub async fn handle(self, client: Client) -> Result<()> {
        match self {
            PortfolioCommand::List {
                output,
                no_headers,
                wide,
            } => {
                let portfolios = client.list_portfolios().await?;
                let columns = output.unwrap_or(vec![PortfolioColumn::Id, PortfolioColumn::Name]);
                let headers = !no_headers;
                Portfolio::print_table(&portfolios, &columns, headers, wide);
            }
            PortfolioCommand::Get {
                id,
                output,
                no_headers,
                wide,
            } => {
                let portfolios = vec![client.get_portfolio(id).await?];
                let columns = output.unwrap_or(vec![PortfolioColumn::Id, PortfolioColumn::Name]);
                let headers = !no_headers;
                Portfolio::print_table(&portfolios, &columns, headers, wide);
            }
            PortfolioCommand::Prices {
                id,
                budget,
                date,
                from,
                codes,
                output,
                no_headers,
                wide,
            } => {
                let mut funds = client
                    .get_portfolio_prices(id, budget, date, from, codes)
                    .await?;
                funds.sort_by(|lhs, rhs| turkish_collate(&lhs.title, &rhs.title));

                let columns = output.unwrap_or(vec![
                    FundToBuyColumn::Title,
                    FundToBuyColumn::Code,
                    FundToBuyColumn::Amount,
                    FundToBuyColumn::Price,
                ]);
                let headers = !no_headers;
                FundToBuy::print_table(&funds, &columns, headers, wide);
            }
            PortfolioCommand::Add {
                id,
                code: fund_code,
                weight,
                min_amount,
            } => {
                let update = PortfolioUpdate {
                    add_codes: {
                        let mut set = HashSet::new();
                        set.insert(PortfolioFundAdd {
                            fund_code,
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
        }

        Ok(())
    }
}
