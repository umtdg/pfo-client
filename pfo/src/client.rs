use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use pfo_core::output::ColumnEnumSorted;
use pfo_core::sort::SortArguments;
use reqwest::header::ACCEPT;
use reqwest::{Client, Method, RequestBuilder, Response, Url};
use serde::Serialize;
use uuid::Uuid;

use crate::cli::FundFilterArgs;
use crate::fund::{FundInfo, FundInfoColumn, FundStats, FundStatsColumn};
use crate::portfolio::{
    Portfolio, PortfolioFund, PortfolioFundColumn, PortfolioFundPrediction,
    PortfolioFundPredictionColumn, PortfolioFundPrice, PortfolioFundPriceColumn, PortfolioUpdate,
};
use crate::problem_detail::ProblemDetail;

pub struct PfoClient {
    client: Client,
    url: Url,
}

impl PfoClient {
    pub fn new(host: String, port: u16) -> Result<Self> {
        let url = format!("http://{}:{}", host, port);
        let url = Url::parse(&url).context(format!("Invalid URL string: {}", url))?;

        log::debug!("Creating client with url {}", url);

        Ok(Self {
            client: Client::new(),
            url,
        })
    }

    fn request<'a, E: ToString, B: Serialize>(
        &self,
        method: Method,
        endpoint: E,
        query: Option<Query<'a>>,
        body: Option<B>,
    ) -> RequestBuilder {
        let mut url = self.url.clone();
        url.set_path(&endpoint.to_string());

        log::debug!("Create request for {}", url);

        let mut request = self.client.request(method, url);

        if let Some(query) = query {
            request = request.query(&query.pairs);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        request
    }

    async fn send_internal(
        &self,
        mut request: RequestBuilder,
        should_have_content: bool,
    ) -> Result<Response> {
        if should_have_content {
            request = request.header(ACCEPT, "application/json");
        }

        let response = request.send().await.context("Failed to send request")?;

        let status = response.status().as_u16();
        log::debug!("Got response {}", status);
        if status >= 400 {
            bail!(match response.json::<ProblemDetail>().await {
                Ok(problem) => format!("{}", problem),
                Err(err) => format!("Error response does not contain ProblemDetail: {:?}", err),
            });
        }

        Ok(response)
    }

    async fn send<'a, E: ToString, B: Serialize>(
        &self,
        method: Method,
        endpoint: E,
        query: Option<Query<'a>>,
        body: Option<B>,
        should_have_content: bool,
    ) -> Result<Response> {
        let request = self.request(method, endpoint, query, body);
        self.send_internal(request, should_have_content).await
    }

    pub async fn list_portfolios(&self) -> Result<Vec<Portfolio>> {
        self.send(Method::GET, "/p", None, none_serialize(), true)
            .await?
            .json()
            .await
            .context("Error when decoding/parsing Portfolio list from response")
    }

    pub async fn get_portfolio(&self, id: Uuid) -> Result<Portfolio> {
        self.send(
            Method::GET,
            format!("/p/{}", id),
            None,
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing Portfolio from response")
    }

    pub async fn get_portfolio_funds(
        &self,
        id: Uuid,
        sort: Option<SortArguments<PortfolioFundColumn>>,
    ) -> Result<Vec<PortfolioFund>> {
        let mut query: Query = Vec::with_capacity(2).into();
        query.push_sort(sort);

        self.send(
            Method::GET,
            format!("/p/{}/f", id),
            Some(query),
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing list of portfolio funds from response")
    }

    pub async fn get_portfolio_fund_prices(
        &self,
        id: Uuid,
        sort: Option<SortArguments<PortfolioFundPriceColumn>>,
    ) -> Result<Vec<PortfolioFundPrice>> {
        let mut query: Query = Vec::with_capacity(2).into();
        query.push_sort(sort);

        self.send(
            Method::GET,
            format!("/p/{}/f/prices", id),
            Some(query),
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing list of portfolio fund prices from response")
    }

    pub async fn get_portfolio_fund_predictions(
        &self,
        id: Uuid,
        budget: f32,
        fund_filter: FundFilterArgs,
        sort: Option<SortArguments<PortfolioFundPredictionColumn>>,
    ) -> Result<Vec<PortfolioFundPrediction>> {
        let mut query: Query = vec![("budget", budget.to_string())].into();
        query.push_fund_filter(fund_filter);
        query.push_sort(sort);

        self.send(
            Method::GET,
            format!("/p/{}/f/predictions", id),
            Some(query),
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing list of portfolio fund predictions from response")
    }

    pub async fn get_portfolio_fund_infos(
        &self,
        id: Uuid,
        sort: Option<SortArguments<FundInfoColumn>>,
    ) -> Result<Vec<FundInfo>> {
        let mut query: Query = Vec::with_capacity(2).into();
        query.push_sort(sort);

        self.send(
            Method::GET,
            format!("/p/{}/f/infos", id),
            Some(query),
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing list of fund informations from response")
    }

    pub async fn get_protfolio_fund_stats(
        &self,
        id: Uuid,
        sort: Option<SortArguments<FundStatsColumn>>,
        force: bool,
    ) -> Result<Vec<FundStats>> {
        let mut query: Query = Vec::with_capacity(3).into();
        query.push_sort(sort);
        query.push_bool("force", force);

        self.send(
            Method::GET,
            format!("/p/{}/f/stats", id),
            Some(query),
            none_serialize(),
            true,
        )
        .await?
        .json()
        .await
        .context("Error when decoding/parsing list of fund stats from response")
    }

    pub async fn update_portfolio(&self, id: Uuid, update: PortfolioUpdate) -> Result<()> {
        self.send(Method::PUT, format!("/p/{}", id), None, Some(update), false)
            .await?;

        Ok(())
    }

    pub async fn get_funds(
        &self,
        fund_filter: FundFilterArgs,
        sort: Option<SortArguments<FundInfoColumn>>,
    ) -> Result<Vec<FundInfo>> {
        let mut query: Query = Vec::with_capacity(5).into();
        query.push_sort(sort);
        query.push_fund_filter(fund_filter);

        self.send(Method::GET, "/f", Some(query), none_serialize(), true)
            .await?
            .json()
            .await
            .context("Error when decoding/parsing list of fund informations from response")
    }

    pub async fn get_fund_stats(
        &self,
        codes: Vec<String>,
        force: bool,
        sort: Option<SortArguments<FundStatsColumn>>,
    ) -> Result<Vec<FundStats>> {
        let mut query: Query = Vec::with_capacity(4).into();
        query.push_sort(sort);
        query.push_bool("force", force);
        query.push_vec("codes", codes);

        self.send(Method::GET, "/f", Some(query), none_serialize(), true)
            .await?
            .json()
            .await
            .context("Error when decoding/parsing list of fund stats from response")
    }
}

struct Query<'a> {
    pub(crate) pairs: Vec<(&'a str, String)>,
}

impl<'a> From<Vec<(&'a str, String)>> for Query<'a> {
    fn from(value: Vec<(&'a str, String)>) -> Self {
        Self { pairs: value }
    }
}

impl<'a> Query<'a> {
    pub fn push_vec(&mut self, key: &'a str, values: Vec<String>) {
        if !values.is_empty() {
            self.pairs.push((key, values.join(",")));
        }
    }

    pub fn push_date(&mut self, key: &'a str, date: Option<NaiveDate>) {
        if let Some(date) = date {
            self.pairs
                .push((key, format!("{}", date.format("%m.%d.%Y"))));
        }
    }

    pub fn push_sort<T: ColumnEnumSorted>(&mut self, sort: Option<SortArguments<T>>) {
        if let Some(sort) = sort {
            self.pairs
                .push(("sortBy", sort.by.to_server_name().to_string()));
            self.pairs.push(("sortDirection", sort.dir.to_string()));
        }
    }

    pub fn push_bool(&mut self, key: &'a str, value: bool) {
        if value {
            self.pairs.push((key, "true".into()));
        }
    }

    pub fn push_fund_filter(&mut self, fund_filter: FundFilterArgs) {
        self.push_date("date", fund_filter.date);
        self.push_date("fetchFrom", fund_filter.from);
        self.push_vec("codes", fund_filter.codes);
    }
}

#[derive(Serialize)]
struct NoneSerialize;

impl NoneSerialize {
    pub fn new() -> Option<Self> {
        None
    }
}

fn none_serialize() -> Option<NoneSerialize> {
    NoneSerialize::new()
}
