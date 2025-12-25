use std::fmt::Display;

use clap::ValueEnum;
use serde::Serialize;

use crate::output::ColumnEnumSorted;

#[derive(Clone, Debug, ValueEnum, Serialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "ASC"),
            SortDirection::Desc => write!(f, "DESC"),
        }
    }
}

impl SortDirection {
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

#[derive(Clone, Debug, Serialize)]
pub struct SortArguments<T: ColumnEnumSorted> {
    #[serde(rename = "sortBy")]
    pub by: T,

    #[serde(rename = "sortDirection")]
    pub dir: SortDirection,
}

impl<T: ColumnEnumSorted> SortArguments<T> {
    pub fn value_parser(s: &str) -> Result<Self, String> {
        let mut parts = s.split_ascii_whitespace();

        Ok(Self {
            by: T::parse_sort_by(parts.next().unwrap_or_default())?,
            dir: SortDirection::value_parser(parts.next().unwrap_or("asc"))?,
        })
    }

    pub fn get_help() -> String {
        format!(
            "<by> <direction> [, <by> <direction>]\nBY: {}\nDIRECTION: {}",
            T::help_sort_by(),
            SortDirection::get_help_string(),
        )
    }
}
