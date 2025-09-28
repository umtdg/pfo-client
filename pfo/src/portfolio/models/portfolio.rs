use std::collections::HashMap;

use pfo_core::trim_string;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::output::{OutputColumn, OutputStruct, OutputTable};

/// Portfolio
#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
}

impl OutputTable for Portfolio {
    type ColumnEnum = PortfolioColumn;
    type OutputStruct = PortfolioOutput;

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
pub enum PortfolioColumn {
    Id,
    Name,
}

impl OutputColumn for PortfolioColumn {
    fn max_len(&self) -> usize {
        match self {
            PortfolioColumn::Id => 36,
            PortfolioColumn::Name => 25,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            PortfolioColumn::Id => "Id",
            PortfolioColumn::Name => "Name",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            PortfolioColumn::Id => true,
            PortfolioColumn::Name => true,
        }
    }
}

pub struct PortfolioOutput {
    pub id: String,
    pub name: String,
}

impl OutputStruct for PortfolioOutput {
    type ColumnEnum = PortfolioColumn;
    type ValueStruct = Portfolio;

    fn from_headers() -> Self {
        Self {
            id: Self::ColumnEnum::Id.name_str().to_string(),
            name: Self::ColumnEnum::Name.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            id: value.id.to_string(),
            name: trim_string(&value.name, Self::ColumnEnum::Name.max_len(), wide),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            PortfolioColumn::Id => &self.id,
            PortfolioColumn::Name => &self.name,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}
