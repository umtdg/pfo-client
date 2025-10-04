use anyhow::Result;
use chrono::{NaiveDate, ParseError};

mod macros;
pub mod output;
pub mod sort;

pub fn trim_string(s: &str, len: usize, wide: bool) -> String {
    if wide {
        s.to_string()
    } else {
        let end = s.char_indices().nth(len).unwrap_or((s.len(), '0')).0;
        s[..end].to_string()
    }
}

pub fn parse_naive_date(s: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(s, "%m.%d.%Y")
}
