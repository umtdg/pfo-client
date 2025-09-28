use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::portfolio::PortfolioFundAdd;

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioUpdate {
    pub add_codes: HashSet<PortfolioFundAdd>,
    pub remove_codes: HashSet<String>,
}
