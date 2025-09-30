use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::trim_string;
use serde::Deserialize;

use crate::cli::SortByEnum;
use crate::output::{OutputColumn, OutputStruct};
use crate::{impl_output_table, impl_sort_by_enum};

#[derive(Debug, Deserialize)]
pub struct FundStats {
    pub code: String,
    pub title: String,
    pub updated_at: NaiveDate,
    pub last_price: f32,
    pub total_value: f32,
    pub daily_return: Option<f32>,
    pub monthly_return: Option<f32>,
    pub three_monthly_return: Option<f32>,
    pub six_monthly_return: Option<f32>,
    pub yearly_return: Option<f32>,
    pub three_yearly_return: Option<f32>,
    pub five_yearly_return: Option<f32>,
}

impl_output_table!(FundStats, FundStatsColumn, FundStatsOutput, FundStatsSortBy);

#[derive(Clone, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
pub enum FundStatsColumn {
    Code,
    Title,
    UpdatedAt,
    LastPrice,
    TotalValue,
    Daily,
    Monthly,
    ThreeMonthly,
    SixMonthly,
    Yearly,
    ThreeYearly,
    FiveYearly,
}

impl OutputColumn for FundStatsColumn {
    fn max_len(&self) -> usize {
        match self {
            Self::Code => 3,
            Self::Title => 25,
            Self::LastPrice => 15,
            Self::UpdatedAt => 10,
            Self::TotalValue => 30,
            Self::Daily => 16,
            Self::Monthly => 16,
            Self::ThreeMonthly => 16,
            Self::SixMonthly => 16,
            Self::Yearly => 16,
            Self::ThreeYearly => 16,
            Self::FiveYearly => 16,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            Self::Code => "Code",
            Self::Title => "Title",
            Self::UpdatedAt => "Updated At",
            Self::LastPrice => "Last Price",
            Self::TotalValue => "Total Value",
            Self::Daily => "Daily",
            Self::Monthly => "Monthly",
            Self::ThreeMonthly => "Three Monthly",
            Self::SixMonthly => "Six Monthly",
            Self::Yearly => "Yearly",
            Self::ThreeYearly => "Three Yearly",
            Self::FiveYearly => "Five Yearly",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            Self::Code => true,
            Self::Title => true,
            Self::UpdatedAt => true,
            Self::LastPrice => false,
            Self::TotalValue => false,
            Self::Daily => true,
            Self::Monthly => true,
            Self::ThreeMonthly => true,
            Self::SixMonthly => true,
            Self::Yearly => true,
            Self::ThreeYearly => true,
            Self::FiveYearly => true,
        }
    }

    fn default_columns() -> Vec<Self> {
        vec![
            Self::Code,
            Self::LastPrice,
            Self::TotalValue,
            Self::Yearly,
            Self::ThreeYearly,
            Self::FiveYearly,
        ]
    }
}

pub struct FundStatsOutput {
    pub code: String,
    pub title: String,
    pub updated_at: String,
    pub last_price: String,
    pub total_value: String,
    pub daily_return: String,
    pub monthly_return: String,
    pub three_monthly_return: String,
    pub six_monthly_return: String,
    pub yearly_return: String,
    pub three_yearly_return: String,
    pub five_yearly_return: String,
}

impl OutputStruct for FundStatsOutput {
    type ColumnEnum = FundStatsColumn;
    type ValueStruct = FundStats;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            updated_at: Self::ColumnEnum::UpdatedAt.name_str().to_string(),
            last_price: Self::ColumnEnum::LastPrice.name_str().to_string(),
            total_value: Self::ColumnEnum::TotalValue.name_str().to_string(),
            daily_return: Self::ColumnEnum::Daily.name_str().to_string(),
            monthly_return: Self::ColumnEnum::Monthly.name_str().to_string(),
            three_monthly_return: Self::ColumnEnum::ThreeMonthly.name_str().to_string(),
            six_monthly_return: Self::ColumnEnum::SixMonthly.name_str().to_string(),
            yearly_return: Self::ColumnEnum::Yearly.name_str().to_string(),
            three_yearly_return: Self::ColumnEnum::ThreeYearly.name_str().to_string(),
            five_yearly_return: Self::ColumnEnum::FiveYearly.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            updated_at: value.updated_at.format("%m.%d.%Y").to_string(),
            last_price: format!("{:.2}", value.last_price),
            total_value: format!("{:.2}", value.total_value),
            daily_return: format!("{:.2}%", value.daily_return.unwrap_or_default()),
            monthly_return: format!("{:.2}%", value.monthly_return.unwrap_or_default()),
            three_monthly_return: format!("{:.2}%", value.three_monthly_return.unwrap_or_default()),
            six_monthly_return: format!("{:.2}%", value.six_monthly_return.unwrap_or_default()),
            yearly_return: format!("{:.2}%", value.yearly_return.unwrap_or_default()),
            three_yearly_return: format!("{:.2}%", value.three_yearly_return.unwrap_or_default()),
            five_yearly_return: format!("{:.2}%", value.five_yearly_return.unwrap_or_default()),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            Self::ColumnEnum::Code => &self.code,
            Self::ColumnEnum::Title => &self.title,
            Self::ColumnEnum::UpdatedAt => &self.updated_at,
            Self::ColumnEnum::LastPrice => &self.last_price,
            Self::ColumnEnum::TotalValue => &self.total_value,
            Self::ColumnEnum::Daily => &self.daily_return,
            Self::ColumnEnum::Monthly => &self.monthly_return,
            Self::ColumnEnum::ThreeMonthly => &self.three_monthly_return,
            Self::ColumnEnum::SixMonthly => &self.six_monthly_return,
            Self::ColumnEnum::Yearly => &self.yearly_return,
            Self::ColumnEnum::ThreeYearly => &self.three_yearly_return,
            Self::ColumnEnum::FiveYearly => &self.five_yearly_return,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum FundStatsSortBy {
    Code,
    Title,
    LastPrice,
    TotalValue,
    DailyReturn,
    MonthlyReturn,
    ThreeMonthlyReturn,
    SixMonthlyReturn,
    YearlyReturn,
    ThreeYearlyReturn,
    FiveYearlyReturn,
}

impl ToString for FundStatsSortBy {
    fn to_string(&self) -> String {
        match self {
            Self::Code => "code",
            Self::Title => "title",
            Self::LastPrice => "lastPrice",
            Self::TotalValue => "totalValue",
            Self::DailyReturn => "dailyReturn",
            Self::MonthlyReturn => "monthlyReturn",
            Self::ThreeMonthlyReturn => "threeMonthlyReturn",
            Self::SixMonthlyReturn => "sixMonthlyReturn",
            Self::YearlyReturn => "yearlyReturn",
            Self::ThreeYearlyReturn => "threeYearlyReturn",
            Self::FiveYearlyReturn => "fiveYearlyReturn",
        }
        .into()
    }
}

impl_sort_by_enum!(FundStatsSortBy);
