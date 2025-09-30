use std::collections::HashMap;

use clap::{Args, ValueEnum};

use crate::cli::{SortArguments, SortByEnum};

pub trait OutputTable: Sized {
    type ColumnEnum: OutputColumn + Send + Sync + 'static;
    type OutputStruct: OutputStruct;
    type SortByEnum: SortByEnum + Send + Sync + 'static;

    const COLUMN_SPACING: usize;

    fn to_value_list(list: &Vec<Self>, headers: bool, wide: bool) -> Vec<Self::OutputStruct>;

    fn calculate_col_widths(
        print_values: &Vec<Self::OutputStruct>,
        columns: &Vec<Self::ColumnEnum>,
    ) -> HashMap<Self::ColumnEnum, usize>;

    fn print_table(list: &Vec<Self>, output: OutputArgs<Self::ColumnEnum, Self::SortByEnum>);
}

pub trait OutputColumn: Sized + ValueEnum {
    fn max_len(&self) -> usize;

    fn name_str(&self) -> &str;

    fn left_align(&self) -> bool;

    fn default_columns() -> Vec<Self>;
}

pub trait OutputStruct {
    type ColumnEnum: OutputColumn;
    type ValueStruct: OutputTable;

    fn from_headers() -> Self;

    fn from_value(value: &Self::ValueStruct, wide: bool) -> Self;

    fn value_from_col(&self, col: &Self::ColumnEnum) -> &str;

    fn len_from_col(&self, col: &Self::ColumnEnum) -> usize;
}

#[derive(Args)]
pub struct OutputArgs<
    Column: Clone + OutputColumn + Send + Sync + 'static,
    SortBy: SortByEnum + Send + Sync + 'static,
> {
    #[arg(
            short,
            long,
            value_parser = SortArguments::<SortBy>::value_parser,
            help = SortArguments::<SortBy>::get_help()
        )]
    pub sort: Option<SortArguments<SortBy>>,

    #[arg(short = 'o', long = "output", value_delimiter = ',')]
    pub columns: Option<Vec<Column>>,

    #[arg(long)]
    pub no_headers: bool,

    #[arg(short, long)]
    pub wide: bool,
}
