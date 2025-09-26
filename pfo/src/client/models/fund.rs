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
