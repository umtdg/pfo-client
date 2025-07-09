use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioUpdate {
    pub add_codes: HashSet<PortfolioFundAdd>,
    pub remove_codes: HashSet<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundToBuy {
    pub code: String,
    pub title: String,
    pub price: f32,
    pub amount: u32,
    pub weight: f32,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct PortfolioFundAdd {
    pub fund_code: String,
    pub weight: u32,
    pub min_amount: u32,
}

impl Hash for PortfolioFundAdd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fund_code.hash(state);
        state.write_u32(self.weight);
        state.write_u32(self.min_amount);
    }
}
