use std::collections::HashMap;

use chrono::NaiveDate;
use clap::ValueEnum;
use pfo_core::trim_string;
use serde::Deserialize;

use crate::cli::SortArgumentEnum;
use crate::output::{OutputColumn, OutputStruct, OutputTable};

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

impl OutputTable for FundInfo {
    type ColumnEnum = FundInfoColumn;
    type OutputStruct = FundInfoOutput;

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
            FundInfoColumn::Code => 3,
            FundInfoColumn::Title => 25,
            FundInfoColumn::Provider => 25,
            FundInfoColumn::Date => 10,
            FundInfoColumn::Price => 15,
            FundInfoColumn::TotalValue => 30,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundInfoColumn::Code => "Code",
            FundInfoColumn::Title => "Title",
            FundInfoColumn::Provider => "Provider",
            FundInfoColumn::Date => "Date",
            FundInfoColumn::Price => "Price",
            FundInfoColumn::TotalValue => "TotalValue",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundInfoColumn::Code => true,
            FundInfoColumn::Title => true,
            FundInfoColumn::Provider => true,
            FundInfoColumn::Date => true,
            FundInfoColumn::Price => false,
            FundInfoColumn::TotalValue => false,
        }
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
            FundInfoSortBy::Code => "code",
            FundInfoSortBy::Title => "title",
            FundInfoSortBy::Provider => "provider",
            FundInfoSortBy::Price => "price",
            FundInfoSortBy::TotalValue => "totalValue",
        }
        .into()
    }
}

impl SortArgumentEnum for FundInfoSortBy {
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
