use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

use crate::item::Item;

pub(crate) fn derive_table(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;
    let column_enum_ident = format_ident!("{}Column", ident);
    let row_struct_ident = format_ident!("{}Row", ident);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("Table derive only supports structs with named fields"),
    };

    let items: Vec<_> = fields
        .named
        .iter()
        .map(|field| {
            Item::from_field(field).unwrap_or_else(|_| panic!("Failed to convert field to Item"))
        })
        .collect();

    let column_enum = gen_column_enum(&column_enum_ident, &items);
    let row_struct = gen_row_struct(&row_struct_ident, &items, ident, &column_enum_ident);

    Ok(quote! {
        #column_enum

        #row_struct
    })
}

fn gen_column_enum(name: &Ident, items: &Vec<Item>) -> TokenStream {
    let mut variants = Vec::with_capacity(items.len());
    let mut header_arms = Vec::with_capacity(items.len());
    let mut max_width_arms = Vec::with_capacity(items.len());
    let mut is_default_arms = Vec::with_capacity(items.len());
    let mut left_align_arms = Vec::with_capacity(items.len());
    let mut sort_by_arms = Vec::with_capacity(items.len());

    for item in items {
        let Item {
            column_variant_ident,
            header,
            max_width,
            is_default,
            left_align,
            sort_by,
            ..
        } = item;

        variants.push(column_variant_ident.clone());
        header_arms.push(quote! { Self::#column_variant_ident => #header });
        max_width_arms.push(quote! { Self::#column_variant_ident => #max_width });
        is_default_arms.push(quote! { Self::#column_variant_ident => #is_default });
        left_align_arms.push(quote! { Self::#column_variant_ident => #left_align });
        sort_by_arms.push(quote! { Self::#column_variant_ident => #sort_by });
    }

    quote! {
        #[derive(Clone, Hash, Eq, PartialEq, clap::ValueEnum)]
        pub enum #name {
            #(#variants),*
        }

        impl pfo_core::output::ColumnEnum for #name {
            fn header(&self) -> &str {
                match self {
                    #(#header_arms),*
                }
            }

            fn max_width(&self) -> usize {
                match self {
                    #(#max_width_arms),*
                }
            }

            fn is_default(&self) -> bool {
                match self {
                    #(#is_default_arms),*
                }
            }

            fn left_align(&self) -> bool {
                match self {
                    #(#left_align_arms),*
                }
            }

            fn default_columns() -> Vec<Self> {
                Self::value_variants()
                    .iter()
                    .filter(|&v| v.is_default())
                    .cloned()
                    .collect()
            }
        }

        impl pfo_core::output::ColumnEnumSorted for #name {
            fn parse_sort_by(s: &str) -> Result<Self, String> {
                Self::from_str(s, true)
            }

            fn help_sort_by() -> String {
                Self::value_variants()
                    .iter()
                    .map(|v| v.to_possible_value().unwrap().get_name().to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            }

            fn to_server_name(&self) -> &str {
                match self {
                    #(#sort_by_arms),*
                }
            }
        }
    }
}

fn gen_row_struct(
    name: &Ident,
    items: &Vec<Item>,
    target_struct: &Ident,
    column_enum: &Ident,
) -> TokenStream {
    let mut row_fields = Vec::with_capacity(items.len());
    let mut from_headers_arms = Vec::with_capacity(items.len());
    let mut from_value_arms = Vec::with_capacity(items.len());
    let mut value_from_col_arms = Vec::with_capacity(items.len());

    for item in items {
        let Item {
            ident,
            column_variant_ident,
            ty,
            should_trim,
            ..
        } = item;

        row_fields.push(quote! { #ident: String });

        from_headers_arms
            .push(quote! {
                #ident: pfo_core::output::ColumnEnum::header(&Self::ColumnEnum::#column_variant_ident).to_string()
            });

        from_value_arms.push(if *should_trim {
            quote! {
                #ident: pfo_core::trim_string(
                    &<#ty as pfo_core::output::ToRowValue>::to_row_value(&value.#ident),
                    pfo_core::output::ColumnEnum::max_width(&Self::ColumnEnum::#column_variant_ident),
                    wide,
                )
            }
        } else {
            quote! {
                #ident: <#ty as pfo_core::output::ToRowValue>::to_row_value(&value.#ident)
            }
        });

        value_from_col_arms
            .push(quote! { Self::ColumnEnum::#column_variant_ident => &self.#ident });
    }

    quote! {
        pub struct #name {
            #(#row_fields),*
        }

        impl pfo_core::output::RowStruct for #name {
            type Target = #target_struct;
            type ColumnEnum = #column_enum;

            fn from_headers() -> Self {
                Self {
                    #(#from_headers_arms),*
                }
            }

            fn from_value(value: &Self::Target, wide: bool) -> Self {
                Self {
                    #(#from_value_arms),*
                }
            }

            fn value_from_col(&self, col: &Self::ColumnEnum) -> &str {
                match col {
                    #(#value_from_col_arms),*
                }
            }

            fn len_from_col(&self, col: &Self::ColumnEnum) -> usize {
                self.value_from_col(col).len()
            }
        }
    }
}
