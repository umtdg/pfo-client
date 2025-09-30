use clap::ValueEnum;
use serde::Serialize;

pub trait SortByEnum: clap::ValueEnum + ToString {
    fn get_help_string() -> String;

    fn value_parser(s: &str) -> Result<Self, String>;
}

#[derive(Clone, Debug, ValueEnum, Serialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl ToString for SortDirection {
    fn to_string(&self) -> String {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
        .into()
    }
}

impl SortByEnum for SortDirection {
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
pub struct SortArguments<T: SortByEnum> {
    #[serde(rename = "sortBy")]
    pub by: T,

    #[serde(rename = "sortDirection")]
    pub dir: SortDirection,
}

impl<T: SortByEnum> SortArguments<T> {
    pub fn value_parser(s: &str) -> Result<Self, String> {
        let mut parts = s.split_ascii_whitespace();

        Ok(Self {
            by: T::value_parser(parts.next().unwrap_or_default())?,
            dir: SortDirection::value_parser(parts.next().unwrap_or("asc"))?,
        })
    }

    pub fn get_help() -> String {
        format!(
            "<by> <direction> [, <by> <direction>]\nBY: {}\nDIRECTION: {}",
            T::get_help_string(),
            SortDirection::get_help_string(),
        )
    }
}
