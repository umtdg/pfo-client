use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FundInformation {
    pub code: String,
    pub title: String,
    pub provider: String,
    pub date: NaiveDate,
    pub price: f32,
    pub total_value: f32,
}
