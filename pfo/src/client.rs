use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::cli::SortArguments;
use crate::config::Config;
use crate::fund::{FundInfoSortBy, FundInfo, FundStats, FundStatsSortBy};
use crate::portfolio::{FundToBuy, Portfolio, PortfolioUpdate};

pub struct Client {
    inner: reqwest::Client,
    host: String,
    port: u16,
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            inner: reqwest::Client::new(),
            host: config.host,
            port: config.port,
        }
    }

    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub async fn list_portfolios(&self) -> Result<Vec<Portfolio>> {
        let url = format!("{}/p", self.base_url());
        let res = self
            .get(url)
            .send()
            .await
            .context("Failed to get portfolio list")?;
        let portfolio_list = res
            .json()
            .await
            .context("Failed to parse portfolio list JSON")?;

        Ok(portfolio_list)
    }

    pub async fn get_portfolio(&self, id: Uuid) -> Result<Portfolio> {
        let url = format!("{}/p/{}", self.base_url(), id);
        let res = self
            .get(url)
            .send()
            .await
            .context("Failed to get portfolio")?;
        let portfolio = res.json().await.context("Failed to parse portfolio JSON")?;

        Ok(portfolio)
    }

    pub async fn get_portfolio_prices(
        &self,
        id: Uuid,
        budget: f32,
        date: Option<NaiveDate>,
        from: Option<NaiveDate>,
        codes: Vec<String>,
    ) -> Result<Vec<FundToBuy>> {
        let url = format!("{}/p/{}/prices", self.base_url(), id);
        let mut req = self.get(url).query(&[("budget", budget)]);

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

    pub async fn update_portfolio(&self, id: Uuid, update: PortfolioUpdate) -> Result<()> {
        let url = format!("{}/p/{}", self.base_url(), id);
        let _res = self
            .put(url)
            .json(&update)
            .send()
            .await
            .context("Failed to update portfolio")?;
        Ok(())
    }

    pub async fn get_funds(
        &self,
        codes: Vec<String>,
        date: Option<NaiveDate>,
        from: Option<NaiveDate>,
        sort: SortArguments<FundInfoSortBy>,
    ) -> Result<Vec<FundInfo>> {
        let url = format!("{}/f", self.base_url());
        let mut req = self.get(url).query(&[
            ("sortBy", sort.by.to_string()),
            ("sortDirection", sort.dir.to_string()),
        ]);

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

    pub async fn get_fund_stats(
        &self,
        codes: Vec<String>,
        force: bool,
        sort: SortArguments<FundStatsSortBy>,
    ) -> Result<Vec<FundStats>> {
        let url = format!("{}/f/stats", self.base_url());
        let mut req = self.get(url).query(&[
            ("sortBy", sort.by.to_string()),
            ("sortDirection", sort.dir.to_string()),
        ]);

        if !codes.is_empty() {
            req = req.query(&[("codes", codes.join(","))]);
        }

        if force {
            req = req.query(&[("force", force)]);
        }

        let res = req.send().await.context("Failed to fetch fund stats")?;

        let stat_list = res
            .json()
            .await
            .context("Failed to parse fund stats JSON")?;

        Ok(stat_list)
    }
}
