use std::collections::HashMap;

use pfo_core::trim_string;

use crate::client::models::{
    fund::{FundInformation, FundStats},
    portfolio::{FundToBuy, Portfolio},
};

pub trait OutputTable: Sized {
    type ColumnEnum: OutputColumn;
    type OutputStruct: OutputStruct;

    const COLUMN_SPACING: usize;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool);
}

pub trait OutputColumn {
    fn max_len(&self) -> usize;

    fn name_str(&self) -> &str;

    fn left_align(&self) -> bool;
}

pub trait OutputStruct {
    type ColumnEnum: OutputColumn;
    type ValueStruct: OutputTable;

    fn from_headers() -> Self;

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self;

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str;

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize;
}

// FundInformation
#[derive(Clone, Eq, Hash, PartialEq, clap::ValueEnum)]
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

pub struct FundInformationOutput {
    pub code: String,
    pub title: String,
    pub provider: String,
    pub date: String,
    pub price: String,
    pub total_value: String,
}

impl OutputStruct for FundInformationOutput {
    type ColumnEnum = FundInformationColumn;
    type ValueStruct = FundInformation;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            provider: Self::ColumnEnum::Provider.name_str().to_string(),
            date: Self::ColumnEnum::Date.name_str().to_string(),
            price: Self::ColumnEnum::Price.name_str().to_string(),
            total_value: Self::ColumnEnum::TotalValue.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            provider: trim_string(&value.provider, Self::ColumnEnum::Provider.max_len(), wide),
            date: format!("{}", value.date.format("%m.%d.%Y")),
            price: format!("{:.2}", value.price),
            total_value: format!("{:.2}", value.total_value),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            Self::ColumnEnum::Code => &self.code,
            Self::ColumnEnum::Title => &self.title,
            Self::ColumnEnum::Provider => &self.provider,
            Self::ColumnEnum::Date => &self.date,
            Self::ColumnEnum::Price => &self.price,
            Self::ColumnEnum::TotalValue => &self.total_value,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

impl OutputTable for FundInformation {
    type ColumnEnum = FundInformationColumn;
    type OutputStruct = FundInformationOutput;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut print_values: Vec<Self::OutputStruct> = vec![];
        if headers {
            print_values.push(Self::OutputStruct::from_headers());
        }

        let mut non_header_values = list
            .iter()
            .map(|item| Self::OutputStruct::from_value(item, wide))
            .collect();
        print_values.append(&mut non_header_values);

        let mut col_widths = HashMap::with_capacity(columns.len());
        for col in columns {
            let max_width = print_values
                .iter()
                .map(|val| val.len_from_col(col))
                .max()
                .unwrap_or(col.max_len());
            col_widths.insert(col.clone(), max_width);
        }

        for f in print_values {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = f.value_from_col(col);

                if col.left_align() {
                    print!("{:<width$}", val, width = width);
                } else {
                    print!("{:>width$}", val, width = width);
                }

                print!("{}", " ".repeat(Self::COLUMN_SPACING));
            }

            println!();
        }
    }
}

// FundStats
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
            FundStatsColumn::Daily => 16,
            FundStatsColumn::Monthly => 16,
            FundStatsColumn::ThreeMonthly => 16,
            FundStatsColumn::SixMonthly => 16,
            FundStatsColumn::Yearly => 16,
            FundStatsColumn::ThreeYearly => 16,
            FundStatsColumn::FiveYearly => 16,
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

pub struct FundStatsOutput {
    pub code: String,
    pub title: String,
    pub last_price: String,
    pub total_value: String,
    pub daily_return: String,
    pub monthly_return: String,
    pub three_monthly_return: String,
    pub six_monthly_return: String,
    pub yearly_return: String,
    pub three_yearly_return: String,
    pub five_yearly_return: String,
}

impl OutputStruct for FundStatsOutput {
    type ColumnEnum = FundStatsColumn;
    type ValueStruct = FundStats;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            last_price: Self::ColumnEnum::LastPrice.name_str().to_string(),
            total_value: Self::ColumnEnum::TotalValue.name_str().to_string(),
            daily_return: Self::ColumnEnum::Daily.name_str().to_string(),
            monthly_return: Self::ColumnEnum::Monthly.name_str().to_string(),
            three_monthly_return: Self::ColumnEnum::ThreeMonthly.name_str().to_string(),
            six_monthly_return: Self::ColumnEnum::SixMonthly.name_str().to_string(),
            yearly_return: Self::ColumnEnum::Yearly.name_str().to_string(),
            three_yearly_return: Self::ColumnEnum::ThreeYearly.name_str().to_string(),
            five_yearly_return: Self::ColumnEnum::FiveYearly.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            last_price: format!("{:.2}", value.last_price),
            total_value: format!("{:.2}", value.total_value),
            daily_return: format!("{:.2}%", value.daily_return.unwrap_or_default()),
            monthly_return: format!("{:.2}%", value.monthly_return.unwrap_or_default()),
            three_monthly_return: format!("{:.2}%", value.three_monthly_return.unwrap_or_default()),
            six_monthly_return: format!("{:.2}%", value.six_monthly_return.unwrap_or_default()),
            yearly_return: format!("{:.2}%", value.yearly_return.unwrap_or_default()),
            three_yearly_return: format!("{:.2}%", value.three_yearly_return.unwrap_or_default()),
            five_yearly_return: format!("{:.2}%", value.five_yearly_return.unwrap_or_default()),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            FundStatsColumn::Code => &self.code,
            FundStatsColumn::Title => &self.title,
            FundStatsColumn::LastPrice => &self.last_price,
            FundStatsColumn::TotalValue => &self.total_value,
            FundStatsColumn::Daily => &self.daily_return,
            FundStatsColumn::Monthly => &self.monthly_return,
            FundStatsColumn::ThreeMonthly => &self.three_monthly_return,
            FundStatsColumn::SixMonthly => &self.six_monthly_return,
            FundStatsColumn::Yearly => &self.yearly_return,
            FundStatsColumn::ThreeYearly => &self.three_yearly_return,
            FundStatsColumn::FiveYearly => &self.five_yearly_return,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

impl OutputTable for FundStats {
    type ColumnEnum = FundStatsColumn;
    type OutputStruct = FundStatsOutput;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut print_values: Vec<Self::OutputStruct> = vec![];
        if headers {
            print_values.push(Self::OutputStruct::from_headers());
        }

        let mut non_header_values = list
            .iter()
            .map(|item| Self::OutputStruct::from_value(item, wide))
            .collect();
        print_values.append(&mut non_header_values);

        let mut col_widths = HashMap::with_capacity(columns.len());
        for col in columns {
            let max_width = print_values
                .iter()
                .map(|val| val.len_from_col(col))
                .max()
                .unwrap_or(col.max_len());
            col_widths.insert(col.clone(), max_width);
        }

        for f in print_values {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = f.value_from_col(col);

                if col.left_align() {
                    print!("{:<width$}", val, width = width);
                } else {
                    print!("{:>width$}", val, width = width);
                }

                print!("{}", " ".repeat(Self::COLUMN_SPACING));
            }

            println!();
        }
    }
}

// Portfolio
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

pub struct PortfolioOutput {
    pub id: String,
    pub name: String,
}

impl OutputStruct for PortfolioOutput {
    type ColumnEnum = PortfolioColumn;
    type ValueStruct = Portfolio;

    fn from_headers() -> Self {
        Self {
            id: Self::ColumnEnum::Id.name_str().to_string(),
            name: Self::ColumnEnum::Name.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            id: value.id.to_string(),
            name: trim_string(&value.name, Self::ColumnEnum::Name.max_len(), wide),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            PortfolioColumn::Id => &self.id,
            PortfolioColumn::Name => &self.name,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

impl OutputTable for Portfolio {
    type ColumnEnum = PortfolioColumn;
    type OutputStruct = PortfolioOutput;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut print_values: Vec<Self::OutputStruct> = vec![];
        if headers {
            print_values.push(Self::OutputStruct::from_headers());
        }

        let mut non_header_values = list
            .iter()
            .map(|item| Self::OutputStruct::from_value(item, wide))
            .collect();
        print_values.append(&mut non_header_values);

        let mut col_widths = HashMap::with_capacity(columns.len());
        for col in columns {
            let max_width = print_values
                .iter()
                .map(|val| val.len_from_col(col))
                .max()
                .unwrap_or(col.max_len());
            col_widths.insert(col.clone(), max_width);
        }

        for f in print_values {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = f.value_from_col(col);

                if col.left_align() {
                    print!("{:<width$}", val, width = width);
                } else {
                    print!("{:>width$}", val, width = width);
                }

                print!("{}", " ".repeat(Self::COLUMN_SPACING));
            }

            println!();
        }
    }
}

// FundToBuy
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

pub struct FundToBuyOutput {
    pub code: String,
    pub title: String,
    pub price: String,
    pub amount: String,
    pub weight: String,
}

impl OutputStruct for FundToBuyOutput {
    type ColumnEnum = FundToBuyColumn;
    type ValueStruct = FundToBuy;

    fn from_headers() -> Self {
        Self {
            code: Self::ColumnEnum::Code.name_str().to_string(),
            title: Self::ColumnEnum::Title.name_str().to_string(),
            price: Self::ColumnEnum::Price.name_str().to_string(),
            amount: Self::ColumnEnum::Amount.name_str().to_string(),
            weight: Self::ColumnEnum::Weight.name_str().to_string(),
        }
    }

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self {
        Self {
            code: trim_string(&value.code, Self::ColumnEnum::Code.max_len(), wide),
            title: trim_string(&value.title, Self::ColumnEnum::Title.max_len(), wide),
            price: format!("{:.2}", value.price),
            amount: value.amount.to_string(),
            weight: format!("{:.2}", value.weight),
        }
    }

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
        match col {
            FundToBuyColumn::Code => &self.code,
            FundToBuyColumn::Title => &self.title,
            FundToBuyColumn::Price => &self.price,
            FundToBuyColumn::Amount => &self.amount,
            FundToBuyColumn::Weight => &self.weight,
        }
    }

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
        self.value_from_col(col).len()
    }
}

impl OutputTable for FundToBuy {
    type ColumnEnum = FundToBuyColumn;
    type OutputStruct = FundToBuyOutput;

    const COLUMN_SPACING: usize = 2;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool) {
        let mut print_values: Vec<Self::OutputStruct> = vec![];
        if headers {
            print_values.push(Self::OutputStruct::from_headers());
        }

        let mut non_header_values = list
            .iter()
            .map(|item| Self::OutputStruct::from_value(item, wide))
            .collect();
        print_values.append(&mut non_header_values);

        let mut col_widths = HashMap::with_capacity(columns.len());
        for col in columns {
            let max_width = print_values
                .iter()
                .map(|val| val.len_from_col(col))
                .max()
                .unwrap_or(col.max_len());
            col_widths.insert(col.clone(), max_width);
        }

        for f in print_values {
            for col in columns {
                let width = col_widths.get(col).unwrap();
                let val = f.value_from_col(col);

                if col.left_align() {
                    print!("{:<width$}", val, width = width);
                } else {
                    print!("{:>width$}", val, width = width);
                }

                print!("{}", " ".repeat(Self::COLUMN_SPACING));
            }

            println!();
        }
    }
}
