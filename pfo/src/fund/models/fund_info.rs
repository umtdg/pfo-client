use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::trim_string;
use serde::Deserialize;

use crate::cli::SortByEnum;
use crate::output::{OutputColumn, OutputStruct};
use crate::{impl_output_table, impl_sort_by_enum};

/// Fund Information
#[derive(Debug, Deserialize)]
pub struct FundInfo {
    pub code: String,
    pub title: String,
    pub provider: String,
    pub date: NaiveDate,
    pub price: f32,
    pub total_value: f32,
}

impl_output_table!(FundInfo, FundInfoColumn, FundInfoOutput, FundInfoSortBy);

#[derive(Clone, Eq, Hash, PartialEq, clap::ValueEnum)]
pub enum FundInfoColumn {
    Code,
    Title,
    Provider,
    Date,
    Price,
    TotalValue,
}

impl OutputColumn for FundInfoColumn {
    fn max_len(&self) -> usize {
        match self {
            Self::Code => 3,
            Self::Title => 25,
            Self::Provider => 25,
            Self::Date => 10,
            Self::Price => 15,
            Self::TotalValue => 30,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            Self::Code => "Code",
            Self::Title => "Title",
            Self::Provider => "Provider",
            Self::Date => "Date",
            Self::Price => "Price",
            Self::TotalValue => "TotalValue",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            Self::Code => true,
            Self::Title => true,
            Self::Provider => true,
            Self::Date => true,
            Self::Price => false,
            Self::TotalValue => false,
        }
    }

    fn default_columns() -> Vec<Self> {
        vec![
            Self::Code,
            Self::Title,
            Self::Date,
            Self::Price,
            Self::TotalValue,
        ]
    }
}

pub struct FundInfoOutput {
    pub code: String,
    pub title: String,
    pub provider: String,
    pub date: String,
    pub price: String,
    pub total_value: String,
}

impl OutputStruct for FundInfoOutput {
    type ColumnEnum = FundInfoColumn;
    type ValueStruct = FundInfo;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            provider: Self::ColumnEnum::Provider.name_str().to_string(),
            date: Self::ColumnEnum::Date.name_str().to_string(),
            price: Self::ColumnEnum::Price.name_str().to_string(),
            total_value: Self::ColumnEnum::TotalValue.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            provider: trim_string(&value.provider, Self::ColumnEnum::Provider.max_len(), wide),
            date: format!("{}", value.date.format("%m.%d.%Y")),
            price: format!("{:.2}", value.price),
            total_value: format!("{:.2}", value.total_value),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            Self::ColumnEnum::Code => &self.code,
            Self::ColumnEnum::Title => &self.title,
            Self::ColumnEnum::Provider => &self.provider,
            Self::ColumnEnum::Date => &self.date,
            Self::ColumnEnum::Price => &self.price,
            Self::ColumnEnum::TotalValue => &self.total_value,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum FundInfoSortBy {
    Code,
    Title,
    Provider,
    Price,
    TotalValue,
}

impl ToString for FundInfoSortBy {
    fn to_string(&self) -> String {
        match self {
            Self::Code => "code",
            Self::Title => "title",
            Self::Provider => "provider",
            Self::Price => "price",
            Self::TotalValue => "totalValue",
        }
        .into()
    }
}

impl_sort_by_enum!(FundInfoSortBy);
