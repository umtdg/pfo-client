use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::derives::derive_table;

mod attr;
mod derives;
mod item;

#[proc_macro_derive(OutputTable, attributes(column))]
pub fn table(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match derive_table(&input) {
        Ok(tokens) => tokens,
        Err(err) => {
            let compile_errors = err.to_compile_error();
            quote! { #compile_errors }
        }
    }
    .into()
}
