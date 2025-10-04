use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::Deserialize;

/// Fund Information
#[derive(Debug, Deserialize, OutputTable)]
pub struct FundInfo {
    #[column(max_width = 3, is_default)]
    pub code: String,

    #[column(max_width = 25, is_default)]
    pub title: String,

    #[column(max_width = 25)]
    pub provider: String,

    #[column(max_width = 10, is_default)]
    pub date: NaiveDate,

    #[column(max_width = 15, is_default, left_align = false)]
    pub price: f32,

    #[column(max_width = 30, left_align = false)]
    pub total_value: f32,
}

impl_table!(FundInfo, FundInfoColumn, FundInfoRow);
