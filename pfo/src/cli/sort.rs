use clap::ValueEnum;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl ToString for SortDirection {
    fn to_string(&self) -> String {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
        .into()
    }
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum SortByFundInfo {
    Code,
    Title,
    Provider,
    Price,
    TotalValue,
}

impl ToString for SortByFundInfo {
    fn to_string(&self) -> String {
        match self {
            SortByFundInfo::Code => "code",
            SortByFundInfo::Title => "title",
            SortByFundInfo::Provider => "provider",
            SortByFundInfo::Price => "price",
            SortByFundInfo::TotalValue => "totalValue",
        }
        .into()
    }
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum SortByFundStats {
    Code,
    Title,
    LastPrice,
    TotalValue,
    DailyReturn,
    MonthlyReturn,
    ThreeMonthlyReturn,
    SixMonthlyReturn,
    YearlyReturn,
    ThreeYearlyReturn,
    FiveYearlyReturn,
}

impl ToString for SortByFundStats {
    fn to_string(&self) -> String {
        match self {
            SortByFundStats::Code => "code",
            SortByFundStats::Title => "title",
            SortByFundStats::LastPrice => "lastPrice",
            SortByFundStats::TotalValue => "totalValue",
            SortByFundStats::DailyReturn => "dailyReturn",
            SortByFundStats::MonthlyReturn => "monthlyReturn",
            SortByFundStats::ThreeMonthlyReturn => "threeMonthlyReturn",
            SortByFundStats::SixMonthlyReturn => "sixMonthlyReturn",
            SortByFundStats::YearlyReturn => "yearlyReturn",
            SortByFundStats::ThreeYearlyReturn => "threeYearlyReturn",
            SortByFundStats::FiveYearlyReturn => "fiveYearlyReturn",
        }.into()
    }
}

#[derive(Clone, Debug)]
pub struct SortArguments<T: clap::ValueEnum + ToString> {
    pub dir: SortDirection,
    pub by: T,
}

impl<T: clap::ValueEnum + ToString> SortArguments<T> {
    pub fn value_parser(s: &str) -> Result<Self, String> {
        let mut parts = s.split_ascii_whitespace();
        let by = T::from_str(parts.next().unwrap_or_default(), true)?;
        let dir = SortDirection::from_str(parts.next().unwrap_or("asc"), true)?;

        Ok(Self { by, dir })
    }

    pub fn help() -> String {
        let possible_by_values: Vec<String> = T::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect();

        let possible_dirs: Vec<String> = SortDirection::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect();

        format!(
            "<by> <direction> [, <by> <direction>]\nBY: {}\nDIRECTION: {}",
            possible_by_values.join(" | "),
            possible_dirs.join(" | ")
        )
    }
}
