use anyhow::{Context, Result};
use uuid::Uuid;

use super::{
    Client,
    models::{FundToBuy, Portfolio, PortfolioUpdate},
};

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
) -> Result<Vec<FundToBuy>> {
    let url = format!("{}/p/{}/prices?budget={}", client.base_url, id, budget);
    let res = client
        .inner
        .get(url)
        .send()
        .await
        .context("Failed to fetch portfolio fund price distribution")?;
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
