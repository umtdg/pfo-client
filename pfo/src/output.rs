use clap::{Args, ValueEnum};

use crate::cli::{SortByEnum, SortArguments};

pub trait OutputTable: Sized {
    type ColumnEnum: OutputColumn;
    type OutputStruct: OutputStruct;

    const COLUMN_SPACING: usize;

    fn print_table(list: &Vec<Self>, columns: &Vec<Self::ColumnEnum>, headers: bool, wide: bool);
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
    Col: Clone + OutputColumn + Send + Sync + 'static,
    By: SortByEnum + Send + Sync + 'static,
> {
    #[arg(
            short,
            long,
            value_parser = SortArguments::<By>::value_parser,
            help = SortArguments::<By>::get_help()
        )]
    pub sort: Option<SortArguments<By>>,

    #[arg(short, long, value_delimiter = ',')]
    pub output: Option<Vec<Col>>,

    #[arg(long)]
    pub no_headers: bool,

    #[arg(short, long)]
    pub wide: bool,
}
