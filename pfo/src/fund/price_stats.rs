use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::Deserialize;

#[derive(Debug, Deserialize, OutputTable)]
pub struct FundPriceStats {
    #[column(max_width = 3, is_default)]
    pub code: String,

    #[column(max_width = 10, is_default)]
    pub date: NaiveDate,

    #[column(max_width = 30, left_align = false, is_default)]
    pub price: f64,

    #[column(max_width = 30, left_align = false, is_default)]
    pub total_value: f64,

    #[column(max_width = 30, left_align = false)]
    pub daily_return: Option<f64>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub monthly_return: Option<f64>,

    #[column(max_width = 30, left_align = false)]
    pub three_monthly_return: Option<f64>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub six_monthly_return: Option<f64>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub yearly_return: Option<f64>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub three_yearly_return: Option<f64>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub five_yearly_return: Option<f64>,
}

impl_table!(FundPriceStats, FundPriceStatsColumn, FundPriceStatsRow);
