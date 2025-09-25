use std::collections::HashMap;

use pfo_core::trim_string;

use crate::client::models::{
    fund::{FundInformation, FundStats},
    portfolio::{FundToBuy, Portfolio},
};

pub trait OutputTable: Sized {
    type ColumnEnum: OutputColumn;

    const COLUMN_SPACING: usize;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool);
}

pub trait OutputColumn {
    fn max_len(&self) -> usize;

    fn name_str(&self) -> &str;

    fn left_align(&self) -> bool;
}

// FundInformation
impl OutputTable for FundInformation {
    type ColumnEnum = FundInformationColumn;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut col_widths = HashMap::new();
        for col in columns {
            let width = match col {
                FundInformationColumn::Code => 3,
                FundInformationColumn::Title => list
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
                FundInformationColumn::Provider => list
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
                FundInformationColumn::Price => list
                    .iter()
                    .map(|f| format!("{:.6}", f.price).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundInformationColumn::TotalValue => list
                    .iter()
                    .map(|f| format!("{:.6}", f.total_value).len())
                    .max()
                    .unwrap_or(col.max_len()),
            };
            col_widths.insert(col.clone(), width + Self::COLUMN_SPACING);
        }

        if headers {
            for col in columns {
                let col_display = col.name_str();
                let col_display_len = col_display.len();
                let mut col_width = *col_widths.get(col).unwrap();
                if col_display_len > col_width - Self::COLUMN_SPACING {
                    col_width = col_display_len;
                    col_widths.insert(col.clone(), col_width + Self::COLUMN_SPACING);
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

        for f in list {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = match col {
                    FundInformationColumn::Code => f.code.clone(),
                    FundInformationColumn::Title => trim_string(&f.title, col.max_len(), wide),
                    FundInformationColumn::Provider => {
                        trim_string(&f.provider, col.max_len(), wide)
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
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
            FundInformationColumn::TotalValue => 30,
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

// FundStats
impl OutputTable for FundStats {
    type ColumnEnum = FundStatsColumn;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut col_widths = HashMap::new();
        for col in columns {
            let width = match col {
                FundStatsColumn::Code => 3,
                FundStatsColumn::Title => list
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
                FundStatsColumn::LastPrice => list
                    .iter()
                    .map(|f| format!("{:.6}", f.last_price).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::TotalValue => list
                    .iter()
                    .map(|f| format!("{:.6}", f.total_value).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::Daily => list
                    .iter()
                    .map(|f| format!("{:.6}", f.daily_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::Monthly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.monthly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::ThreeMonthly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.three_monthly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::SixMonthly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.six_monthly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::Yearly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.yearly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::ThreeYearly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.three_yearly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundStatsColumn::FiveYearly => list
                    .iter()
                    .map(|f| format!("{:.6}", f.five_yearly_return.unwrap_or_default()).len())
                    .max()
                    .unwrap_or(col.max_len()),
            };
            col_widths.insert(col.clone(), width + Self::COLUMN_SPACING);
        }

        if headers {
            for col in columns {
                let col_display = col.name_str();
                let col_display_len = col_display.len();
                let mut col_width = *col_widths.get(col).unwrap();
                if col_display_len > col_width - Self::COLUMN_SPACING {
                    col_width = col_display_len;
                    col_widths.insert(col.clone(), col_width + Self::COLUMN_SPACING);
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

        for f in list {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = match col {
                    FundStatsColumn::Code => f.code.clone(),
                    FundStatsColumn::Title => trim_string(&f.title, col.max_len(), wide),
                    FundStatsColumn::LastPrice => format!("{:.6}", f.last_price),
                    FundStatsColumn::TotalValue => format!("{:.6}", f.total_value),
                    FundStatsColumn::Daily => format!("{:.2}%", f.daily_return.unwrap_or_default()),
                    FundStatsColumn::Monthly => {
                        format!("{:.2}%", f.monthly_return.unwrap_or_default())
                    }
                    FundStatsColumn::ThreeMonthly => {
                        format!("{:.2}%", f.three_monthly_return.unwrap_or_default())
                    }
                    FundStatsColumn::SixMonthly => {
                        format!("{:.2}%", f.six_monthly_return.unwrap_or_default())
                    }
                    FundStatsColumn::Yearly => {
                        format!("{:.2}%", f.yearly_return.unwrap_or_default())
                    }
                    FundStatsColumn::ThreeYearly => {
                        format!("{:.2}%", f.three_yearly_return.unwrap_or_default())
                    }
                    FundStatsColumn::FiveYearly => {
                        format!("{:.2}%", f.five_yearly_return.unwrap_or_default())
                    }
                };

                if col.left_align() {
                    print!("{:<width$}", val, width = width)
                } else {
                    print!("{:>width$}", val, width = width)
                }
            }

            println!();
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
pub enum FundStatsColumn {
    Code,
    Title,
    LastPrice,
    TotalValue,
    Daily,
    Monthly,
    ThreeMonthly,
    SixMonthly,
    Yearly,
    ThreeYearly,
    FiveYearly,
}

impl OutputColumn for FundStatsColumn {
    fn max_len(&self) -> usize {
        match self {
            FundStatsColumn::Code => 3,
            FundStatsColumn::Title => 25,
            FundStatsColumn::LastPrice => 15,
            FundStatsColumn::TotalValue => 30,
            FundStatsColumn::Daily => 30,
            FundStatsColumn::Monthly => 30,
            FundStatsColumn::ThreeMonthly => 30,
            FundStatsColumn::SixMonthly => 30,
            FundStatsColumn::Yearly => 30,
            FundStatsColumn::ThreeYearly => 30,
            FundStatsColumn::FiveYearly => 30,
        }
    }

    fn name_str(&self) -> &str {
        match self {
            FundStatsColumn::Code => "Code",
            FundStatsColumn::Title => "Title",
            FundStatsColumn::LastPrice => "LastPrice",
            FundStatsColumn::TotalValue => "TotalValue",
            FundStatsColumn::Daily => "Daily",
            FundStatsColumn::Monthly => "Monthly",
            FundStatsColumn::ThreeMonthly => "ThreeMonthly",
            FundStatsColumn::SixMonthly => "SixMonthly",
            FundStatsColumn::Yearly => "Yearly",
            FundStatsColumn::ThreeYearly => "ThreeYearly",
            FundStatsColumn::FiveYearly => "FiveYearly",
        }
    }

    fn left_align(&self) -> bool {
        match self {
            FundStatsColumn::Code => true,
            FundStatsColumn::Title => true,
            FundStatsColumn::LastPrice => false,
            FundStatsColumn::TotalValue => false,
            FundStatsColumn::Daily => true,
            FundStatsColumn::Monthly => true,
            FundStatsColumn::ThreeMonthly => true,
            FundStatsColumn::SixMonthly => true,
            FundStatsColumn::Yearly => true,
            FundStatsColumn::ThreeYearly => true,
            FundStatsColumn::FiveYearly => true,
        }
    }
}

// Portfolio
impl OutputTable for Portfolio {
    type ColumnEnum = PortfolioColumn;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut col_widths = HashMap::new();
        for col in columns {
            let width = match col {
                PortfolioColumn::Id => 36,
                PortfolioColumn::Name => list
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
            col_widths.insert(col.clone(), width + Self::COLUMN_SPACING);
        }

        if headers {
            for col in columns {
                let col_display = col.name_str();

                let col_display_len = col_display.len();
                let mut col_width = *col_widths.get(col).unwrap();
                if col_display_len + Self::COLUMN_SPACING > col_width {
                    col_width = col_display_len;
                    col_widths.insert(col.clone(), col_width + Self::COLUMN_SPACING);
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

        for p in list {
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
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

// FundToBuy
impl OutputTable for FundToBuy {
    type ColumnEnum = FundToBuyColumn;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut col_widths = HashMap::new();
        for col in columns {
            let width = match col {
                FundToBuyColumn::Code => 3,
                FundToBuyColumn::Title => list
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
                FundToBuyColumn::Price => list
                    .iter()
                    .map(|f| format!("{:.6}", f.price).len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundToBuyColumn::Amount => list
                    .iter()
                    .map(|f| f.amount.to_string().len())
                    .max()
                    .unwrap_or(col.max_len()),
                FundToBuyColumn::Weight => list
                    .iter()
                    .map(|f| format!("{:.6}", f.weight).len())
                    .max()
                    .unwrap_or(col.max_len()),
            };
            col_widths.insert(col.clone(), width + Self::COLUMN_SPACING);
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
                if col_display_len > col_width - Self::COLUMN_SPACING {
                    col_width = col_display_len;
                    col_widths.insert(col.clone(), col_width + Self::COLUMN_SPACING);
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

        for f in list {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = match col {
                    FundToBuyColumn::Code => f.code.clone(),
                    FundToBuyColumn::Title => trim_string(&f.title, col.max_len(), wide),
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, clap::ValueEnum)]
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
