use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::Deserialize;

#[derive(Debug, Deserialize, OutputTable)]
pub struct FundStats {
    #[column(max_width = 3, is_default)]
    pub code: String,

    #[column(max_width = 25)]
    pub title: String,

    #[column(max_width = 10)]
    pub updated_at: NaiveDate,

    #[column(max_width = 15, left_align = false, is_default)]
    pub last_price: f32,

    #[column(max_width = 30, left_align = false, is_default)]
    pub total_value: f32,

    #[column(max_width = 30, left_align = false)]
    pub daily_return: Option<f32>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub monthly_return: Option<f32>,

    #[column(max_width = 30, left_align = false)]
    pub three_monthly_return: Option<f32>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub six_monthly_return: Option<f32>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub yearly_return: Option<f32>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub three_yearly_return: Option<f32>,

    #[column(max_width = 30, left_align = false, is_default)]
    pub five_yearly_return: Option<f32>,
}

impl_table!(FundStats, FundStatsColumn, FundStatsRow);
