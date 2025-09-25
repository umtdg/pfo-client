use anyhow::{Context, Result};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::client::Client;
use crate::client::models::portfolio::{FundToBuy, Portfolio, PortfolioUpdate};

pub async fn get_portfolio(client: &Client, id: Uuid) -> Result<Portfolio> {
    let url = format!("{}/p/{}", client.base_url, id);
    let res = client
        .inner
        .get(url)
        .send()
        .await
        .context("Failed to fetch portfolio")?;
    let portfolio = res.json().await.context("Failed to parse portfolio JSON")?;

    Ok(portfolio)
}

pub async fn get_portfolios(client: &Client) -> Result<Vec<Portfolio>> {
    let url = format!("{}/p", client.base_url);
    let res = client
        .inner
        .get(url)
        .send()
        .await
        .context("Failed to fetch portfolio list")?;
    let portfolio_list = res
        .json()
        .await
        .context("Failed to parse portfolio list JSON")?;

    Ok(portfolio_list)
}

pub async fn get_portfolio_prices(
    client: &Client,
    id: Uuid,
    budget: f32,
    date: Option<NaiveDate>,
    from: Option<NaiveDate>,
    codes: Vec<String>,
) -> Result<Vec<FundToBuy>> {
    let url = format!("{}/p/{}/prices", client.base_url, id);
    let mut req = client.inner.get(url).query(&[("budget", budget)]);

    if let Some(date) = date {
        req = req.query(&[("date", &format!("{}", date.format("%m.%d.%Y")))]);
    }

    if let Some(from) = from {
        req = req.query(&[("fetchFrom", &format!("{}", from.format("%m.%d.%Y")))]);
    }

    if !codes.is_empty() {
        req = req.query(&[("codes", codes.join(","))]);
    }

    let res = req.send().await.context("Failed to fetch fund prices")?;

    let funds = res
        .json()
        .await
        .context("Failed to parse portfolio fund price distribution JSON")?;

    Ok(funds)
}

pub async fn update_portfolio(client: &Client, id: Uuid, update: PortfolioUpdate) -> Result<()> {
    let url = format!("{}/p/{}", client.base_url, id);
    let _res = client
        .inner
        .put(url)
        .json(&update)
        .send()
        .await
        .context("Failed to update portfolio")?;
    Ok(())
}
