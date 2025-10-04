use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::{Deserialize, Serialize};

/// FundToBuy
#[derive(Debug, Deserialize, Serialize, OutputTable)]
pub struct FundToBuy {
    #[column(max_width = 3, is_default)]
    pub code: String,

    #[column(max_width = 25, is_default)]
    pub title: String,

    #[column(max_width = 15, is_default)]
    pub price: f32,

    #[column(max_width = 10, is_default)]
    pub amount: u32,

    #[column(max_width = 15, is_default)]
    pub weight: f32,
}

impl_table!(FundToBuy, FundToBuyColumn, FundToBuyRow);
