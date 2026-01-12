use chrono::NaiveDate;
use pfo_core::{output::ColumnEnumSorted, sort::SortArguments};

use crate::cli::FundFilterArgs;

pub(crate) struct Query<'a> {
    pairs: Vec<(&'a str, String)>,
}

impl<'a> From<Vec<(&'a str, String)>> for Query<'a> {
    fn from(pairs: Vec<(&'a str, String)>) -> Self {
        Query { pairs }
    }
}

impl<'a> Query<'a> {
    pub fn pairs(&self) -> &Vec<(&'a str, String)> {
        &self.pairs
    }

    pub fn push_vec(&mut self, key: &'a str, values: Vec<String>) {
        if !values.is_empty() {
            self.pairs.push((key, values.join(",")));
        }
    }

    pub fn push_date(&mut self, key: &'a str, date: Option<NaiveDate>) {
        if let Some(date) = date {
            self.pairs
                .push((key, format!("{}", date.format("%m.%d.%Y"))));
        }
    }

    pub fn push_sort<T: ColumnEnumSorted>(&mut self, sort: Option<SortArguments<T>>) {
        if let Some(sort) = sort {
            self.pairs
                .push(("sortBy", sort.by.to_server_name().to_string()));
            self.pairs.push(("sortDirection", sort.dir.to_string()));
        }
    }

    pub fn push_fund_filter(&mut self, fund_filter: FundFilterArgs) {
        self.push_date("date", fund_filter.date);
        self.push_vec("codes", fund_filter.codes);
    }
}
