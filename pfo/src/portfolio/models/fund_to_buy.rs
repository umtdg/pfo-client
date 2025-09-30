use std::collections::HashMap;

use clap::ValueEnum;
use pfo_core::trim_string;
use serde::{Deserialize, Serialize};

use crate::{
    cli::SortByEnum,
    output::{OutputColumn, OutputStruct, OutputTable},
};

/// FundToBuy
#[derive(Debug, Deserialize, Serialize)]
pub struct FundToBuy {
    pub code: String,
    pub title: String,
    pub price: f32,
    pub amount: u32,
    pub weight: f32,
}

impl OutputTable for FundToBuy {
    type ColumnEnum = FundToBuyColumn;
    type OutputStruct = FundToBuyOutput;

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
pub enum FundToBuyColumn {
    Code,
    Title,
    Price,
    Amount,
    Weight,
}

impl OutputColumn for FundToBuyColumn {
    fn max_len(&self) -> usize {
        match self {
            FundToBuyColumn::Code => 3,
            FundToBuyColumn::Title => 25,
            FundToBuyColumn::Price => 15,
            FundToBuyColumn::Amount => 10,
            FundToBuyColumn::Weight => 10,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundToBuyColumn::Code => "Code",
            FundToBuyColumn::Title => "Title",
            FundToBuyColumn::Price => "Price",
            FundToBuyColumn::Amount => "Amount",
            FundToBuyColumn::Weight => "Weight",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundToBuyColumn::Code => true,
            FundToBuyColumn::Title => true,
            FundToBuyColumn::Price => false,
            FundToBuyColumn::Amount => false,
            FundToBuyColumn::Weight => false,
        }
    }

    fn default_columns() -> Vec<Self> {
        vec![Self::Title, Self::Code, Self::Amount, Self::Price]
    }
}

pub struct FundToBuyOutput {
    pub code: String,
    pub title: String,
    pub price: String,
    pub amount: String,
    pub weight: String,
}

impl OutputStruct for FundToBuyOutput {
    type ColumnEnum = FundToBuyColumn;
    type ValueStruct = FundToBuy;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            price: Self::ColumnEnum::Price.name_str().to_string(),
            amount: Self::ColumnEnum::Amount.name_str().to_string(),
            weight: Self::ColumnEnum::Weight.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            price: format!("{:.2}", value.price),
            amount: value.amount.to_string(),
            weight: format!("{:.2}", value.weight),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            Self::ColumnEnum::Code => &self.code,
            Self::ColumnEnum::Title => &self.title,
            Self::ColumnEnum::Price => &self.price,
            Self::ColumnEnum::Amount => &self.amount,
            Self::ColumnEnum::Weight => &self.weight,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

#[derive(Clone, ValueEnum)]
pub enum FundToBuySortBy {
    Code,
    Title,
    Price,
    Amount,
    Weight,
}

impl ToString for FundToBuySortBy {
    fn to_string(&self) -> String {
        match self {
            Self::Code => "code",
            Self::Title => "title",
            Self::Price => "price",
            Self::Amount => "amount",
            Self::Weight => "weight",
        }
        .into()
    }
}

impl SortByEnum for FundToBuySortBy {
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
