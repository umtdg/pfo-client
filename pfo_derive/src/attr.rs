use quote::ToTokens;
use syn::{Attribute, Ident, LitBool, LitInt, LitStr, Token, parse::Parse, punctuated::Punctuated};

pub(crate) struct Attr {
    pub(crate) name: Option<AttrName>,
    pub(crate) value: Option<AttrValue>,
}

impl Attr {
    pub(crate) fn parse_all(all_attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error> {
        let mut parsed = Vec::new();
        for attr in all_attrs {
            if !attr.path().is_ident("column") {
                continue;
            }

            for attr in attr.parse_args_with(Punctuated::<Attr, Token![,]>::parse_terminated)? {
                parsed.push(attr);
            }
        }

        Ok(parsed)
    }

    pub(crate) fn value_or_abort(&self) -> Result<&AttrValue, syn::Error> {
        self.value
            .as_ref()
            .ok_or_else(|| panic!("attribute `{:?}` reqruies a value", self.name))
    }

    pub(crate) fn lit_str_or_abort(&self) -> Result<&LitStr, syn::Error> {
        let value = self.value_or_abort()?;
        match value {
            AttrValue::Str(t) => Ok(t),
            _ => {
                panic!(
                    "attribute `{:?}` can only accept string literals",
                    self.name
                )
            }
        }
    }

    pub(crate) fn lit_int_or_abort(&self) -> Result<&LitInt, syn::Error> {
        let value = self.value_or_abort()?;
        match value {
            AttrValue::Int(t) => Ok(t),
            _ => {
                panic!(
                    "attribute `{:?}` can only accept string literals",
                    self.name
                )
            }
        }
    }

    pub(crate) fn lit_bool_or_abort(&self) -> Result<&LitBool, syn::Error> {
        let value = self.value_or_abort()?;
        match value {
            AttrValue::Bool(t) => Ok(t),
            _ => {
                panic!(
                    "attribute `{:?}` can only accept string literals",
                    self.name
                )
            }
        }
    }
}

impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident_str = input.parse::<Ident>()?.to_string();

        let name = match ident_str.as_str() {
            "header" => Some(AttrName::Header),
            "max_width" => Some(AttrName::MaxWidth),
            "is_default" => Some(AttrName::IsDefault),
            "left_align" => Some(AttrName::LeftAlign),
            "sort_by" => Some(AttrName::SortBy),
            _ => None,
        };

        let value = if input.peek(Token![=]) {
            let _ = input.parse::<Token![=]>()?; // skip `=`
            if input.peek(LitStr) {
                Some(AttrValue::Str(input.parse()?))
            } else if input.peek(LitInt) {
                Some(AttrValue::Int(input.parse()?))
            } else if input.peek(LitBool) {
                Some(AttrValue::Bool(input.parse()?))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self { name, value })
    }
}

#[derive(Debug)]
pub(crate) enum AttrName {
    Header,
    MaxWidth,
    IsDefault,
    LeftAlign,
    SortBy,
}

pub(crate) enum AttrValue {
    Str(LitStr),
    Bool(LitBool),
    Int(LitInt),
}

impl ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrValue::Str(t) => t.to_tokens(tokens),
            AttrValue::Bool(t) => t.to_tokens(tokens),
            AttrValue::Int(t) => t.to_tokens(tokens),
        }
    }
}
