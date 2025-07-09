use std::{cmp::Ordering, collections::HashSet};

use anyhow::{Context, Result};

use crate::{
    cli::args::PortfolioCommand,
    client::{
        Client,
        models::{PortfolioFundAdd, PortfolioUpdate},
    },
    output::{FundToBuyColumn, PortfolioColumn},
};

fn turkish_char_order(c: char) -> usize {
    match c {
        'A' => 0,
        'a' => 1,
        'B' => 2,
        'b' => 3,
        'C' => 4,
        'c' => 5,
        'Ç' => 6,
        'ç' => 7,
        'D' => 8,
        'd' => 9,
        'E' => 10,
        'e' => 11,
        'F' => 12,
        'f' => 13,
        'G' => 14,
        'g' => 15,
        'Ğ' => 16,
        'ğ' => 17,
        'H' => 18,
        'h' => 19,
        'I' => 20,
        'ı' => 21,
        'İ' => 22,
        'i' => 23,
        'J' => 24,
        'j' => 25,
        'K' => 26,
        'k' => 27,
        'L' => 28,
        'l' => 29,
        'M' => 30,
        'm' => 31,
        'N' => 32,
        'n' => 33,
        'O' => 34,
        'o' => 35,
        'Ö' => 36,
        'ö' => 37,
        'P' => 38,
        'p' => 39,
        'Q' => 40,
        'q' => 41,
        'R' => 42,
        'r' => 43,
        'S' => 44,
        's' => 45,
        'Ş' => 46,
        'ş' => 47,
        'T' => 48,
        't' => 49,
        'U' => 50,
        'u' => 51,
        'Ü' => 52,
        'ü' => 53,
        'V' => 54,
        'v' => 55,
        'W' => 56,
        'w' => 57,
        'X' => 58,
        'x' => 59,
        'Y' => 60,
        'y' => 61,
        'Z' => 62,
        'z' => 63,
        _ => usize::MAX,
    }
}

fn turkish_collate(lhs: &str, rhs: &str) -> Ordering {
    for (lc, rc) in lhs.chars().zip(rhs.chars()) {
        match turkish_char_order(lc).cmp(&turkish_char_order(rc)) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    lhs.len().cmp(&rhs.len())
}

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
            output,
            no_headers,
            wide,
        } => {
            let mut funds = client.get_portfolio_prices(id, budget).await?;
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
