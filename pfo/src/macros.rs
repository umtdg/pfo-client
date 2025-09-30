#[macro_export]
macro_rules! impl_output_table {
    ($ty: ty, $col_ty: ty, $out_ty: ty, $by_ty: ty) => {
        impl crate::output::OutputTable for $ty {
            type ColumnEnum = $col_ty;
            type OutputStruct = $out_ty;
            type SortByEnum = $by_ty;

            const COLUMN_SPACING: usize = 2;

            fn to_value_list(
                list: &Vec<Self>,
                headers: bool,
                wide: bool,
            ) -> Vec<Self::OutputStruct> {
                let mut print_values: Vec<Self::OutputStruct> = vec![];
                if headers {
                    print_values.push(Self::OutputStruct::from_headers());
                }

                let mut non_header_values = list
                    .iter()
                    .map(|item| Self::OutputStruct::from_value(item, wide))
                    .collect();
                print_values.append(&mut non_header_values);

                print_values
            }

            fn calculate_col_widths(
                print_values: &Vec<Self::OutputStruct>,
                columns: &Vec<Self::ColumnEnum>,
            ) -> std::collections::HashMap<Self::ColumnEnum, usize> {
                let mut col_widths = std::collections::HashMap::with_capacity(columns.len());
                for col in columns {
                    let max_width = print_values
                        .iter()
                        .map(|val| val.len_from_col(col))
                        .max()
                        .unwrap_or(col.max_len());
                    col_widths.insert(col.clone(), max_width);
                }

                col_widths
            }

            fn print_table(
                list: &Vec<Self>,
                output: crate::output::OutputArgs<Self::ColumnEnum, Self::SortByEnum>,
            ) {
                let print_values = Self::to_value_list(list, !output.no_headers, output.wide);

                let columns = output
                    .columns
                    .unwrap_or(Self::ColumnEnum::default_columns());
                let col_widths = Self::calculate_col_widths(&print_values, &columns);

                for f in print_values {
                    for col in &columns {
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
    };
}

#[macro_export]
macro_rules! impl_sort_by_enum {
    ($ty: ty) => {
        impl SortByEnum for $ty {
            fn get_help_string() -> String {
                Self::value_variants()
                    .iter()
                    .map(|v| v.to_possible_value().unwrap().get_name().to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            }

            fn value_parser(s: &str) -> Result<Self, String> {
                Self::from_str(s, true)
            }
        }
    };
}
