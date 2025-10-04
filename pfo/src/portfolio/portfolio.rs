use clap::ValueEnum;
use pfo_core::impl_table;
use pfo_derive::OutputTable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Portfolio
#[derive(Debug, Deserialize, Serialize, OutputTable)]
pub struct Portfolio {
    #[column(max_width = 36, is_default)]
    pub id: Uuid,

    #[column(max_width = 50, is_default)]
    pub name: String,
}

impl_table!(Portfolio, PortfolioColumn, PortfolioRow);
