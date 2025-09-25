use anyhow::{Context, Result};
use chrono::NaiveDate;

use crate::client::Client;
use crate::client::models::fund::{FundInformation, FundStats};

pub async fn get_fund(
    client: &Client,
    code: String,
    date: Option<NaiveDate>,
) -> Result<FundInformation> {
    _ = client;
    _ = code;
    _ = date;

    unimplemented!("Getting single fund is not implemented yet")
}

pub async fn get_funds(
    client: &Client,
    codes: Vec<String>,
    date: Option<NaiveDate>,
    from: Option<NaiveDate>,
) -> Result<Vec<FundInformation>> {
    let url = format!("{}/f", client.base_url);
    let mut req = client.inner.get(url);

    if !codes.is_empty() {
        req = req.query(&[("codes", codes.join(","))]);
    }

    if let Some(date) = date {
        req = req.query(&[("date", &format!("{}", date.format("%m.%d.%Y")))]);
    }

    if let Some(from) = from {
        req = req.query(&[("fetchFrom", &format!("{}", from.format("%m.%d.%Y")))]);
    }

    let res = req.send().await.context("Failed to fetch funds")?;

    let fund_list = res.json().await.context("Failed to parse fund list JSON")?;

    Ok(fund_list)
}

pub async fn get_fund_stats(client: &Client, codes: Vec<String>) -> Result<Vec<FundStats>> {
    let url = format!("{}/f/stats", client.base_url);
    let mut req = client.inner.get(url);

    if !codes.is_empty() {
        req = req.query(&[("codes", codes.join(","))]);
    }

    let res = req.send().await.context("Failed to fetch fund stats")?;

    let stat_list = res
        .json()
        .await
        .context("Failed to parse fund stats JSON")?;

    Ok(stat_list)
}
