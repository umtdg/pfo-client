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
