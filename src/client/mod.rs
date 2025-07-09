pub mod endpoints;
pub mod models;

use anyhow::Result;
use models::{FundToBuy, Portfolio, PortfolioUpdate};
use uuid::Uuid;

use crate::config::Config;

pub struct Client {
    inner: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            inner: reqwest::Client::new(),
            base_url: format!("http://{}:{}", &config.host, &config.port),
        }
    }

    pub async fn list_portfolios(&self) -> Result<Vec<Portfolio>> {
        endpoints::get_portfolios(self).await
    }

    pub async fn get_portfolio(&self, id: Uuid) -> Result<Portfolio> {
        endpoints::get_portfolio(self, id).await
    }

    pub async fn get_portfolio_prices(&self, id: Uuid, budget: f32) -> Result<Vec<FundToBuy>> {
        endpoints::get_portfolio_prices(self, id, budget).await
    }

    pub async fn update_portfolio(&self, id: Uuid, update: PortfolioUpdate) -> Result<()> {
        endpoints::update_portfolio(self, id, update).await
    }
}
