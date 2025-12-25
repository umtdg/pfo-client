#[macro_export]
macro_rules! impl_table {
    ($ty: ty, $col_ty: ty, $row_ty: ty) => {
        impl pfo_core::output::Table for $ty {
            type ColumnEnum = $col_ty;
            type RowStruct = $row_ty;

            const COLUMN_SPACING: usize = 4;

            fn print_table(list: &[Self], opts: pfo_core::output::TableArgs<Self::ColumnEnum>) {
                let columns = opts.columns.unwrap_or(
                    <Self::ColumnEnum as pfo_core::output::ColumnEnum>::default_columns(),
                );

                let mut values: Vec<Self::RowStruct> = Vec::with_capacity(list.len());
                if !opts.no_headers {
                    values.push(<Self::RowStruct as pfo_core::output::RowStruct>::from_headers());
                }

                values.append(
                    &mut list
                        .iter()
                        .map(|item| {
                            <Self::RowStruct as pfo_core::output::RowStruct>::from_value(
                                item, opts.wide,
                            )
                        })
                        .collect(),
                );

                let widths: std::collections::HashMap<Self::ColumnEnum, usize> = columns
                    .iter()
                    .map(|c| {
                        let width = values
                            .iter()
                            .map(|v| {
                                <Self::RowStruct as pfo_core::output::RowStruct>::len_from_col(v, c)
                            })
                            .max()
                            .unwrap_or(
                                <Self::ColumnEnum as pfo_core::output::ColumnEnum>::max_width(c),
                            );
                        (c.clone(), width)
                    })
                    .collect();

                for val in values {
                    for col in &columns {
                        let width = widths.get(&col).unwrap();
                        if <Self::ColumnEnum as pfo_core::output::ColumnEnum>::left_align(col) {
                            print!(
                                "{:<width$}",
                                <Self::RowStruct as pfo_core::output::RowStruct>::value_from_col(
                                    &val, col
                                ),
                                width = width
                            );
                        } else {
                            print!(
                                "{:>width$}",
                                <Self::RowStruct as pfo_core::output::RowStruct>::value_from_col(
                                    &val, col
                                ),
                                width = width
                            );
                        }

                        print!("{}", " ".repeat(Self::COLUMN_SPACING));
                    }

                    println!();
                }
            }
        }
    };
}
