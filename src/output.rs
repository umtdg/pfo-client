use std::collections::HashMap;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::client::models::{
    fund::FundInformation,
    portfolio::{FundToBuy, Portfolio},
};

trait OutputColumn {
    fn max_len(&self) -> usize;

    fn name_str(&self) -> &str;

    fn left_align(&self) -> bool;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum PortfolioColumn {
    Id,
    Name,
}

impl OutputColumn for PortfolioColumn {
    fn max_len(&self) -> usize {
        match self {
            PortfolioColumn::Id => 36,
            PortfolioColumn::Name => 25,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            PortfolioColumn::Id => "Id",
            PortfolioColumn::Name => "Name",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            PortfolioColumn::Id => true,
            PortfolioColumn::Name => true,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum FundToBuyColumn {
    Code,
    Title,
    Price,
    Amount,
    Weight,
}

impl OutputColumn for FundToBuyColumn {
    fn max_len(&self) -> usize {
        match self {
            FundToBuyColumn::Code => 3,
            FundToBuyColumn::Title => 25,
            FundToBuyColumn::Price => 15,
            FundToBuyColumn::Amount => 10,
            FundToBuyColumn::Weight => 10,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundToBuyColumn::Code => "Code",
            FundToBuyColumn::Title => "Title",
            FundToBuyColumn::Price => "Price",
            FundToBuyColumn::Amount => "Amount",
            FundToBuyColumn::Weight => "Weight",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundToBuyColumn::Code => true,
            FundToBuyColumn::Title => true,
            FundToBuyColumn::Price => false,
            FundToBuyColumn::Amount => false,
            FundToBuyColumn::Weight => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ValueEnum)]
pub enum FundInformationColumn {
    Code,
    Title,
    Provider,
    Date,
    Price,
    TotalValue,
}

impl OutputColumn for FundInformationColumn {
    fn max_len(&self) -> usize {
        match self {
            FundInformationColumn::Code => 3,
            FundInformationColumn::Title => 25,
            FundInformationColumn::Provider => 25,
            FundInformationColumn::Date => 10,
            FundInformationColumn::Price => 15,
            FundInformationColumn::TotalValue => 15,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundInformationColumn::Code => "Code",
            FundInformationColumn::Title => "Title",
            FundInformationColumn::Provider => "Provider",
            FundInformationColumn::Date => "Date",
            FundInformationColumn::Price => "Price",
            FundInformationColumn::TotalValue => "TotalValue",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundInformationColumn::Code => true,
            FundInformationColumn::Title => true,
            FundInformationColumn::Provider => true,
            FundInformationColumn::Date => true,
            FundInformationColumn::Price => false,
            FundInformationColumn::TotalValue => false,
        }
    }
}

pub fn print_portfolios(
    portfolios: &Vec<Portfolio>,
    columns: &Vec<PortfolioColumn>,
    headers: bool,
    wide: bool,
) {
    const COL_SPACING: usize = 2;

    let mut col_widths = HashMap::new();
    for col in columns {
        let width = match col {
            PortfolioColumn::Id => 36,
            PortfolioColumn::Name => portfolios
                .iter()
                .map(|p| {
                    let len = p.name.len();
                    if wide {
                        len
                    } else {
                        std::cmp::min(len, col.max_len())
                    }
                })
                .max()
                .unwrap_or(col.max_len()),
        };
        col_widths.insert(col.clone(), width + COL_SPACING);
    }

    if headers {
        for col in columns {
            let col_display = col.name_str();

            let col_display_len = col_display.len();
            let mut col_width = *col_widths.get(col).unwrap();
            if col_display_len + COL_SPACING > col_width {
                col_width = col_display_len;
                col_widths.insert(col.clone(), col_width + COL_SPACING);
            }

            if col.left_align() {
                print!(
                    "{:<width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            } else {
                print!(
                    "{:>width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            }
        }
        println!();
    }

    for p in portfolios {
        for col in columns {
            let width = col_widths.get(col).unwrap();
            let val = match col {
                PortfolioColumn::Id => p.id.to_string(),
                PortfolioColumn::Name => p.name.clone(),
            };
            if col.left_align() {
                print!("{:<width$}", val, width = width);
            } else {
                print!("{:>width$}", val, width = width);
            }
        }

        println!();
    }
}

pub fn print_fund_buy_prices(
    funds: &Vec<FundToBuy>,
    columns: &Vec<FundToBuyColumn>,
    headers: bool,
    wide: bool,
) {
    const COL_SPACING: usize = 2;

    let mut col_widths = HashMap::new();
    for col in columns {
        let width = match col {
            FundToBuyColumn::Code => 3,
            FundToBuyColumn::Title => funds
                .iter()
                .map(|f| {
                    let len = f.title.len();
                    if wide {
                        len
                    } else {
                        std::cmp::min(len, col.max_len())
                    }
                })
                .max()
                .unwrap_or(col.max_len()),
            FundToBuyColumn::Price => funds
                .iter()
                .map(|f| format!("{:.6}", f.price).len())
                .max()
                .unwrap_or(col.max_len()),
            FundToBuyColumn::Amount => funds
                .iter()
                .map(|f| f.amount.to_string().len())
                .max()
                .unwrap_or(col.max_len()),
            FundToBuyColumn::Weight => funds
                .iter()
                .map(|f| format!("{:.6}", f.weight).len())
                .max()
                .unwrap_or(col.max_len()),
        };
        col_widths.insert(col.clone(), width + COL_SPACING);
    }

    if headers {
        for col in columns {
            let col_display = match col {
                FundToBuyColumn::Code => "Code",
                FundToBuyColumn::Title => "Title",
                FundToBuyColumn::Price => "Price",
                FundToBuyColumn::Amount => "Amount",
                FundToBuyColumn::Weight => "Weight",
            };
            let col_display_len = col_display.len();
            let mut col_width = *col_widths.get(col).unwrap();
            if col_display_len > col_width - COL_SPACING {
                col_width = col_display_len;
                col_widths.insert(col.clone(), col_width + COL_SPACING);
            }

            if col.left_align() {
                print!(
                    "{:<width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            } else {
                print!(
                    "{:>width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            }
        }
        println!();
    }

    for f in funds {
        for col in columns {
            let width = col_widths.get(col).unwrap();
            let val = match col {
                FundToBuyColumn::Code => f.code.clone(),
                FundToBuyColumn::Title => {
                    if wide {
                        f.title.clone()
                    } else {
                        let end = f
                            .title
                            .char_indices()
                            .nth(col.max_len())
                            .unwrap_or((f.title.len(), '0'))
                            .0;
                        f.title[..end].to_string()
                    }
                }
                FundToBuyColumn::Price => format!("{:.6}", f.price),
                FundToBuyColumn::Amount => f.amount.to_string(),
                FundToBuyColumn::Weight => format!("{:.6}", f.weight),
            };
            if col.left_align() {
                print!("{:<width$}", val, width = width);
            } else {
                print!("{:>width$}", val, width = width);
            }
        }

        println!();
    }
}

pub fn print_fund_infos(
    fund_infos: &Vec<FundInformation>,
    columns: &Vec<FundInformationColumn>,
    headers: bool,
    wide: bool,
) {
    const COL_SPACING: usize = 2;

    let mut col_widths = HashMap::new();
    for col in columns {
        let width = match col {
            FundInformationColumn::Code => 3,
            FundInformationColumn::Title => fund_infos
                .iter()
                .map(|f| {
                    let len = f.title.len();
                    if wide {
                        len
                    } else {
                        std::cmp::min(len, col.max_len())
                    }
                })
                .max()
                .unwrap_or(col.max_len()),
            FundInformationColumn::Provider => fund_infos
                .iter()
                .map(|f| {
                    let len = f.provider.len();
                    if wide {
                        len
                    } else {
                        std::cmp::min(len, col.max_len())
                    }
                })
                .max()
                .unwrap_or(col.max_len()),
            FundInformationColumn::Date => 10,
            FundInformationColumn::Price => fund_infos
                .iter()
                .map(|f| format!("{:.6}", f.price).len())
                .max()
                .unwrap_or(col.max_len()),
            FundInformationColumn::TotalValue => fund_infos
                .iter()
                .map(|f| format!("{:.6}", f.price).len())
                .max()
                .unwrap_or(col.max_len()),
        };
        col_widths.insert(col.clone(), width + COL_SPACING);
    }

    if headers {
        for col in columns {
            let col_display = col.name_str();
            let col_display_len = col_display.len();
            let mut col_width = *col_widths.get(col).unwrap();
            if col_display_len > col_width - COL_SPACING {
                col_width = col_display_len;
                col_widths.insert(col.clone(), col_width + COL_SPACING);
            }

            if col.left_align() {
                print!(
                    "{:<width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            } else {
                print!(
                    "{:>width$}",
                    col_display,
                    width = col_widths.get(col).unwrap()
                );
            }
        }
        println!();
    }

    for f in fund_infos {
        for col in columns {
            let width = col_widths.get(col).unwrap();
            let val = match col {
                FundInformationColumn::Code => f.code.clone(),
                FundInformationColumn::Title => {
                    if wide {
                        f.title.clone()
                    } else {
                        let end = f
                            .title
                            .char_indices()
                            .nth(col.max_len())
                            .unwrap_or((f.title.len(), '0'))
                            .0;
                        f.title[..end].to_string()
                    }
                }
                FundInformationColumn::Provider => {
                    if wide {
                        f.provider.clone()
                    } else {
                        let end = f
                            .provider
                            .char_indices()
                            .nth(col.max_len())
                            .unwrap_or((f.provider.len(), '0'))
                            .0;
                        f.provider[..end].to_string()
                    }
                }
                FundInformationColumn::Date => format!("{}", f.date.format("%m.%d.%Y")),
                FundInformationColumn::Price => format!("{:.6}", f.price),
                FundInformationColumn::TotalValue => format!("{:.6}", f.total_value),
            };

            if col.left_align() {
                print!("{:<width$}", val, width = width);
            } else {
                print!("{:>width$}", val, width = width);
            }
        }

        println!();
    }
}
