mod prediction;
mod price;
mod update;

pub use prediction::{PortfolioFundPrediction, PortfolioFundPredictionColumn};
pub use price::{PortfolioFundPrice, PortfolioFundPriceColumn};
pub use update::PortfolioFundUpdate;

use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, OutputTable)]
pub struct PortfolioFund {
    #[column(header = "Code", max_width = 3, is_default)]
    pub fund_code: String,

    #[column(max_width = 36)]
    pub portfolio_id: Uuid,

    #[column(max_width = 15)]
    pub weight: f32,

    #[column(max_width = 15, is_default)]
    pub normalized_weight: f32,

    #[column(max_width = 10)]
    pub min_amount: u32,

    #[column(max_width = 10, is_default)]
    pub owned_amount: u32,

    #[column(header = "Spent", max_width = 30, is_default)]
    pub total_money_spent: f64,
}

impl_table!(PortfolioFund, PortfolioFundColumn, PortfolioFundRow);
