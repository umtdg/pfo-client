use clap::ValueEnum;
use pfo_core::trim_string;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cli::SortByEnum;
use crate::output::{OutputColumn, OutputStruct};
use crate::{impl_output_table, impl_sort_by_enum};

/// Portfolio
#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
}

impl_output_table!(Portfolio, PortfolioColumn, PortfolioOutput, PortfolioSortBy);

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

    fn default_columns() -> Vec<Self> {
        vec![PortfolioColumn::Id, PortfolioColumn::Name]
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

#[derive(Clone, ValueEnum)]
pub enum PortfolioSortBy {
    Id,
    Name,
}

impl ToString for PortfolioSortBy {
    fn to_string(&self) -> String {
        match self {
            Self::Id => "id",
            Self::Name => "name",
        }
        .into()
    }
}

impl_sort_by_enum!(PortfolioSortBy);
