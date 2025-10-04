use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct PortfolioFundAdd {
    pub fund_code: String,
    pub weight: u32,
    pub min_amount: u32,
}

impl Hash for PortfolioFundAdd {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fund_code.hash(state);
        state.write_u32(self.weight);
        state.write_u32(self.min_amount);
    }
}
