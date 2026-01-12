use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioFundUpdate {
    pub fund_code: String,
    pub weight: Option<u32>,
    pub min_amount: Option<u32>,
    pub owned_amount: Option<u32>,
    pub total_money_spent: Option<f64>,
}

impl Hash for PortfolioFundUpdate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fund_code.hash(state);
    }
}

impl Eq for PortfolioFundUpdate {}

impl PartialEq for PortfolioFundUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.fund_code == other.fund_code
    }
}
