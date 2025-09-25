pub mod endpoints;
pub mod models;

use anyhow::Result;
use chrono::NaiveDate;
use models::portfolio::{FundToBuy, Portfolio, PortfolioUpdate};
use uuid::Uuid;

use crate::client::models::fund::FundInformation;
use crate::client::models::fund::FundStats;

use super::config::Config;
use endpoints::fund::*;
use endpoints::portfolio::*;

pub struct Client {
    pub(crate) inner: reqwest::Client,
    pub(crate) base_url: String,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            inner: reqwest::Client::new(),
            base_url: format!("http://{}:{}", &config.host, &config.port),
        }
    }

    pub async fn list_portfolios(&self) -> Result<Vec<Portfolio>> {
        get_portfolios(self).await
    }

    pub async fn get_portfolio(&self, id: Uuid) -> Result<Portfolio> {
        get_portfolio(self, id).await
    }

    pub async fn get_portfolio_prices(
        &self,
        id: Uuid,
        budget: f32,
        date: Option<NaiveDate>,
        from: Option<NaiveDate>,
        codes: Vec<String>,
    ) -> Result<Vec<FundToBuy>> {
        get_portfolio_prices(self, id, budget, date, from, codes).await
    }

    pub async fn update_portfolio(&self, id: Uuid, update: PortfolioUpdate) -> Result<()> {
        update_portfolio(self, id, update).await
    }

    pub async fn get_fund(&self, code: String, date: Option<NaiveDate>) -> Result<FundInformation> {
        get_fund(self, code, date).await
    }

    pub async fn get_funds(
        &self,
        codes: Vec<String>,
        date: Option<NaiveDate>,
        from: Option<NaiveDate>,
    ) -> Result<Vec<FundInformation>> {
        get_funds(self, codes, date, from).await
    }

    pub async fn get_fund_stats(&self, codes: Vec<String>) -> Result<Vec<FundStats>> {
        get_fund_stats(self, codes).await
    }
}
