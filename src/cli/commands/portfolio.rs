use std::collections::HashSet;

use crate::turkish::*;
use anyhow::{Context, Result};

use crate::{
    cli::args::PortfolioCommand,
    client::{
        Client,
        models::portfolio::{PortfolioFundAdd, PortfolioUpdate},
    },
    output::{FundToBuyColumn, PortfolioColumn},
};

pub async fn handle(cmd: PortfolioCommand, client: Client) -> Result<()> {
    match cmd {
        PortfolioCommand::List {
            output,
            no_headers,
            wide,
        } => {
            let portfolios = client.list_portfolios().await?;
            let columns = output.unwrap_or(vec![PortfolioColumn::Id, PortfolioColumn::Name]);
            let headers = !no_headers;
            crate::output::print_portfolios(&portfolios, &columns, headers, wide);
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
            crate::output::print_portfolios(&portfolios, &columns, headers, wide);
        }
        PortfolioCommand::Prices {
            id,
            budget,
            date,
            output,
            no_headers,
            wide,
        } => {
            let mut funds = client.get_portfolio_prices(id, budget, date).await?;
            funds.sort_by(|lhs, rhs| turkish_collate(&lhs.title, &rhs.title));

            let columns = output.unwrap_or(vec![
                FundToBuyColumn::Title,
                FundToBuyColumn::Code,
                FundToBuyColumn::Amount,
                FundToBuyColumn::Price,
            ]);
            let headers = !no_headers;
            crate::output::print_fund_buy_prices(&funds, &columns, headers, wide);
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
