use clap::{Args, ValueEnum};
use chrono::NaiveDate;
use uuid::Uuid;

pub trait Table: Sized {
    type ColumnEnum: ColumnEnum + Send + Sync + 'static;
    type RowStruct: RowStruct;

    const COLUMN_SPACING: usize;

    fn print_table(list: &Vec<Self>, opts: TableArgs<Self::ColumnEnum>);
}

pub trait ColumnEnum: Sized + ValueEnum {
    fn max_width(&self) -> usize;

    fn header(&self) -> &str;

    fn left_align(&self) -> bool;

    fn is_default(&self) -> bool;

    fn default_columns() -> Vec<Self>;
}

pub trait ColumnEnumSorted: ColumnEnum {
    fn parse_sort_by(s: &str) -> Result<Self, String>;

    fn help_sort_by() -> String;

    fn to_server_name(&self) -> &str;
}

pub trait RowStruct {
    type ColumnEnum: ColumnEnum;
    type Target: Table;

    fn from_headers() -> Self;

    fn from_value(value: &Self::Target, wide: bool) -> Self;

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str;

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize;
}

pub trait ToRowValue {
    fn to_row_value(&self) -> String;
}

impl ToRowValue for String {
    fn to_row_value(&self) -> String {
        self.clone()
    }
}

impl ToRowValue for u32 {
    fn to_row_value(&self) -> String {
        self.to_string()
    }
}

impl ToRowValue for f32 {
    fn to_row_value(&self) -> String {
        format!("{:.2}", self)
    }
}

impl ToRowValue for NaiveDate {
    fn to_row_value(&self) -> String {
        self.format("%m.%d.%Y").to_string()
    }
}

impl ToRowValue for Uuid {
    fn to_row_value(&self) -> String {
        self.to_string()
    }
}

impl<T: ToRowValue> ToRowValue for Option<T> {
    fn to_row_value(&self) -> String {
        match self {
            Some(v) => v.to_row_value(),
            None => "-".into(),
        }
    }
}

#[derive(Args)]
pub struct TableArgs<T: Clone + ColumnEnum + Send + Sync + 'static> {
    #[arg(short = 'o', long = "output", value_delimiter = ',')]
    pub columns: Option<Vec<T>>,

    #[arg(long)]
    pub no_headers: bool,

    #[arg(short, long)]
    pub wide: bool,
}
