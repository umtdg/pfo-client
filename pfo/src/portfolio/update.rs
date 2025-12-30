use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::portfolio::PortfolioFundUpdate;

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioUpdate {
    pub add_codes: HashSet<PortfolioFundUpdate>,
    pub remove_codes: HashSet<String>,
}
