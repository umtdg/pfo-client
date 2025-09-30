use std::collections::HashMap;

use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::trim_string;
use serde::Deserialize;

use crate::{
    cli::SortByEnum,
    output::{OutputColumn, OutputStruct, OutputTable},
};

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

impl OutputTable for FundStats {
    type ColumnEnum = FundStatsColumn;
    type OutputStruct = FundStatsOutput;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut print_values: Vec<Self::OutputStruct> = vec![];
        if headers {
            print_values.push(Self::OutputStruct::from_headers());
        }

        let mut non_header_values = list
            .iter()
            .map(|item| Self::OutputStruct::from_value(item, wide))
            .collect();
        print_values.append(&mut non_header_values);

        let mut col_widths = HashMap::with_capacity(columns.len());
        for col in columns {
            let max_width = print_values
                .iter()
                .map(|val| val.len_from_col(col))
                .max()
                .unwrap_or(col.max_len());
            col_widths.insert(col.clone(), max_width);
        }

        for f in print_values {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = f.value_from_col(col);

                if col.left_align() {
                    print!("{:<width$}", val, width = width);
                } else {
                    print!("{:>width$}", val, width = width);
                }

                print!("{}", " ".repeat(Self::COLUMN_SPACING));
            }

            println!();
        }
    }
}

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
            FundStatsColumn::Code => 3,
            FundStatsColumn::Title => 25,
            FundStatsColumn::LastPrice => 15,
            FundStatsColumn::UpdatedAt => 10,
            FundStatsColumn::TotalValue => 30,
            FundStatsColumn::Daily => 16,
            FundStatsColumn::Monthly => 16,
            FundStatsColumn::ThreeMonthly => 16,
            FundStatsColumn::SixMonthly => 16,
            FundStatsColumn::Yearly => 16,
            FundStatsColumn::ThreeYearly => 16,
            FundStatsColumn::FiveYearly => 16,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundStatsColumn::Code => "Code",
            FundStatsColumn::Title => "Title",
            FundStatsColumn::UpdatedAt => "Updated At",
            FundStatsColumn::LastPrice => "Last Price",
            FundStatsColumn::TotalValue => "Total Value",
            FundStatsColumn::Daily => "Daily",
            FundStatsColumn::Monthly => "Monthly",
            FundStatsColumn::ThreeMonthly => "Three Monthly",
            FundStatsColumn::SixMonthly => "Six Monthly",
            FundStatsColumn::Yearly => "Yearly",
            FundStatsColumn::ThreeYearly => "Three Yearly",
            FundStatsColumn::FiveYearly => "Five Yearly",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundStatsColumn::Code => true,
            FundStatsColumn::Title => true,
            FundStatsColumn::UpdatedAt => true,
            FundStatsColumn::LastPrice => false,
            FundStatsColumn::TotalValue => false,
            FundStatsColumn::Daily => true,
            FundStatsColumn::Monthly => true,
            FundStatsColumn::ThreeMonthly => true,
            FundStatsColumn::SixMonthly => true,
            FundStatsColumn::Yearly => true,
            FundStatsColumn::ThreeYearly => true,
            FundStatsColumn::FiveYearly => true,
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
            FundStatsColumn::Code => &self.code,
            FundStatsColumn::Title => &self.title,
            FundStatsColumn::UpdatedAt => &self.updated_at,
            FundStatsColumn::LastPrice => &self.last_price,
            FundStatsColumn::TotalValue => &self.total_value,
            FundStatsColumn::Daily => &self.daily_return,
            FundStatsColumn::Monthly => &self.monthly_return,
            FundStatsColumn::ThreeMonthly => &self.three_monthly_return,
            FundStatsColumn::SixMonthly => &self.six_monthly_return,
            FundStatsColumn::Yearly => &self.yearly_return,
            FundStatsColumn::ThreeYearly => &self.three_yearly_return,
            FundStatsColumn::FiveYearly => &self.five_yearly_return,
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
            FundStatsSortBy::Code => "code",
            FundStatsSortBy::Title => "title",
            FundStatsSortBy::LastPrice => "lastPrice",
            FundStatsSortBy::TotalValue => "totalValue",
            FundStatsSortBy::DailyReturn => "dailyReturn",
            FundStatsSortBy::MonthlyReturn => "monthlyReturn",
            FundStatsSortBy::ThreeMonthlyReturn => "threeMonthlyReturn",
            FundStatsSortBy::SixMonthlyReturn => "sixMonthlyReturn",
            FundStatsSortBy::YearlyReturn => "yearlyReturn",
            FundStatsSortBy::ThreeYearlyReturn => "threeYearlyReturn",
            FundStatsSortBy::FiveYearlyReturn => "fiveYearlyReturn",
        }
        .into()
    }
}

impl SortByEnum for FundStatsSortBy {
    fn get_help_string() -> String {
        Self::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect::<Vec<String>>()
            .join(" | ")
    }

    fn value_parser(s: &str) -> Result<Self, String> {
        Self::from_str(s, true)
    }
}
