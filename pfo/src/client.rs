use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use pfo_core::output::ColumnEnumSorted;
use pfo_core::sort::SortArguments;
use reqwest::IntoUrl;
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::cli::FundFilterArgs;
use crate::config::Config;
use crate::fund::{FundInfo, FundInfoColumn, FundStats, FundStatsColumn};
use crate::portfolio::{FundToBuy, Portfolio, PortfolioUpdate};
use crate::problem_detail::ProblemDetail;

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

    fn endpoint_url<T: ToString>(&self, endpoint: T) -> String {
        format!("http://{}:{}{}", self.host, self.port, endpoint.to_string())
    }

    fn portfolio_endpoint_url<T: ToString>(&self, endpoint: T, id: Uuid) -> String {
        format!(
            "http://{}:{}/p/{}{}",
            self.host,
            self.port,
            id,
            endpoint.to_string()
        )
    }

    pub async fn get_with_problem_detail<SuccessDto, Url, Query>(
        &self,
        url: Url,
        query: Query,
    ) -> Result<SuccessDto>
    where
        SuccessDto: DeserializeOwned,
        Url: IntoUrl,
        Query: Serialize,
    {
        let req = self.get(url).query(&query);

        let res = req.send().await.context("GET failed")?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            anyhow::bail!(format!("{}", res.json::<ProblemDetail>().await?))
        }
    }

    pub async fn put_with_problem_detail<SuccessDto, Url, Query, BodyDto>(
        &self,
        url: Url,
        query: Query,
        body: &BodyDto,
    ) -> Result<SuccessDto>
    where
        SuccessDto: DeserializeOwned,
        Url: IntoUrl,
        Query: Serialize,
        BodyDto: Serialize,
    {
        let req = self.get(url).query(&query).json(body);

        let res = req.send().await.context("PUT failed")?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            anyhow::bail!(format!("{}", res.json::<ProblemDetail>().await?))
        }
    }

    pub async fn list_portfolios(&self) -> Result<Vec<Portfolio>> {
        self.get_with_problem_detail(self.endpoint_url("/p"), ())
            .await
    }

    pub async fn get_portfolio(&self, id: Uuid) -> Result<Portfolio> {
        self.get_with_problem_detail(self.portfolio_endpoint_url("", id), ())
            .await
    }

    pub async fn get_portfolio_prices(
        &self,
        id: Uuid,
        budget: f32,
        fund_filter: FundFilterArgs,
    ) -> Result<Vec<FundToBuy>> {
        let mut query: Vec<(&str, String)> = vec![("budget", budget.to_string())];
        query_push_fund_filter(&mut query, fund_filter);

        self.get_with_problem_detail(self.portfolio_endpoint_url("/prices", id), query)
            .await
    }

    pub async fn get_portfolio_fund_infos(
        &self,
        id: Uuid,
        sort: &Option<SortArguments<FundInfoColumn>>,
    ) -> Result<Vec<FundInfo>> {
        let mut query: Vec<(&str, String)> = Vec::with_capacity(2);
        query_push_sort(&mut query, sort);

        self.get_with_problem_detail(self.portfolio_endpoint_url("/info", id), query)
            .await
    }

    pub async fn get_protfolio_fund_stats(
        &self,
        id: Uuid,
        sort: &Option<SortArguments<FundStatsColumn>>,
        force: bool,
    ) -> Result<Vec<FundStats>> {
        let mut query: Vec<(&str, String)> = Vec::with_capacity(3);
        query_push_sort(&mut query, sort);
        query_push_bool(&mut query, "force", force);

        self.get_with_problem_detail(self.portfolio_endpoint_url("/stats", id), query)
            .await
    }

    pub async fn update_portfolio(&self, id: Uuid, update: PortfolioUpdate) -> Result<()> {
        self.put_with_problem_detail(self.portfolio_endpoint_url("", id), (), &update)
            .await
    }

    pub async fn get_funds(
        &self,
        fund_filter: FundFilterArgs,
        sort: &Option<SortArguments<FundInfoColumn>>,
    ) -> Result<Vec<FundInfo>> {
        let mut query: Vec<(&str, String)> = Vec::with_capacity(5);
        query_push_sort(&mut query, sort);
        query_push_fund_filter(&mut query, fund_filter);

        self.get_with_problem_detail(self.endpoint_url("/f"), query)
            .await
    }

    pub async fn get_fund_stats(
        &self,
        codes: Vec<String>,
        force: bool,
        sort: &Option<SortArguments<FundStatsColumn>>,
    ) -> Result<Vec<FundStats>> {
        let mut query: Vec<(&str, String)> = Vec::with_capacity(4);
        query_push_sort(&mut query, sort);
        query_push_bool(&mut query, "force", force);
        query_push_vec(&mut query, "codes", codes);

        self.get_with_problem_detail(self.endpoint_url("/f/stats"), query)
            .await
    }
}

fn query_push_vec<'a>(query: &mut Vec<(&'a str, String)>, key: &'a str, values: Vec<String>) {
    if !values.is_empty() {
        query.push((key, values.join(",")));
    }
}

fn query_push_date<'a>(query: &mut Vec<(&'a str, String)>, key: &'a str, date: Option<NaiveDate>) {
    if let Some(date) = date {
        query.push((key, format!("{}", date.format("%m.%d.%Y"))));
    }
}

fn query_push_sort<'a, T: ColumnEnumSorted>(
    query: &mut Vec<(&'a str, String)>,
    sort: &Option<SortArguments<T>>,
) {
    if let Some(sort) = sort {
        query.push(("sortBy", sort.by.to_server_name().to_string()));
        query.push(("sortDirection", sort.dir.to_string()));
    }
}

fn query_push_bool<'a>(query: &mut Vec<(&'a str, String)>, key: &'a str, value: bool) {
    if value {
        query.push((key, "true".into()));
    }
}

fn query_push_fund_filter(query: &mut Vec<(&str, String)>, fund_filter: FundFilterArgs) {
    query_push_date(query, "date", fund_filter.date);
    query_push_date(query, "fetchFrom", fund_filter.from);
    query_push_vec(query, "codes", fund_filter.codes);
}
