use heck::{ToLowerCamelCase, ToPascalCase, ToTitleCase};
use syn::{Field, GenericArgument, Ident, PathArguments, PathSegment, Type};

use crate::attr::{Attr, AttrName};

static OPTION_PATH_IDENTS: &[&str] = &["Option|", "std|option|Option|", "core|option|Option|"];
static STRING_PATH_IDENTS: &[&str] = &["String|", "std|string|String|"];

pub(crate) struct Item {
    pub(crate) ident: Ident,
    pub(crate) column_variant_ident: Ident,
    pub(crate) ty: Type,
    pub(crate) header: String,
    pub(crate) max_width: usize,
    pub(crate) is_default: bool,
    pub(crate) left_align: bool,
    pub(crate) sort_by: String,
    pub(crate) should_trim: bool,
}

fn ty_extract_generic(segment: &PathSegment) -> Type {
    match segment.arguments {
        PathArguments::AngleBracketed(ref params) => {
            let generic_arg = params
                .args
                .first()
                .unwrap_or_else(|| panic!("Empty generic argument list"));

            match generic_arg {
                GenericArgument::Type(ty) => ty.clone(),
                GenericArgument::AssocType(assoc_type) => assoc_type.ty.clone(),
                _ => panic!("Only generic arguments with Type or Associated Type are supported"),
            }
        }
        _ => panic!("Cannot extract generic type without angled brackets"),
    }
}

fn ty_should_trim(ty: &Type) -> bool {
    match ty {
        Type::Array(_) | Type::Slice(_) | Type::Tuple(_) => true,
        Type::Paren(type_paren) => ty_should_trim(&type_paren.elem),
        Type::Path(type_path) => match &type_path.qself {
            Some(_) => todo!("With QSelf"),
            None => {
                let path_idents = type_path.path.segments.iter().into_iter().fold(
                    String::new(),
                    |mut acc, seg| {
                        acc.push_str(&seg.ident.to_string());
                        acc.push('|');
                        acc
                    },
                );

                let option_seg = OPTION_PATH_IDENTS
                    .into_iter()
                    .find(|s| &path_idents == *s)
                    .and_then(|_| type_path.path.segments.last());

                if let Some(option_seg) = option_seg {
                    return ty_should_trim(&ty_extract_generic(option_seg));
                }

                STRING_PATH_IDENTS
                    .into_iter()
                    .find(|s| &path_idents == *s)
                    .and_then(|_| type_path.path.segments.last())
                    .is_some()
            }
        },
        Type::Ptr(type_ptr) => ty_should_trim(&type_ptr.elem),
        Type::Reference(type_reference) => ty_should_trim(&type_reference.elem),
        _ => false,
    }
}

impl Item {
    pub(crate) fn new(ident: Ident, ty: Type) -> Self {
        let name_str = ident.to_string();

        let column_variant_ident = Ident::new(&name_str.to_pascal_case(), ident.span());
        let should_trim = ty_should_trim(&ty);

        Self {
            ident,
            ty,
            header: name_str.to_title_case(),
            max_width: 0,
            is_default: false,
            left_align: true,
            sort_by: name_str.to_lower_camel_case(),
            column_variant_ident,
            should_trim,
        }
    }

    pub(crate) fn from_field(field: &Field) -> Result<Self, syn::Error> {
        let ident = field.ident.as_ref().unwrap().clone();
        let ty = field.ty.clone();

        let mut res = Self::new(ident, ty);
        let parsed_attrs = Attr::parse_all(&field.attrs)?;
        res.push_attrs(&parsed_attrs)?;

        Ok(res)
    }

    fn push_attrs(&mut self, attrs: &[Attr]) -> Result<(), syn::Error> {
        let mut max_width_seen = false;

        for attr in attrs {
            match &attr.name {
                Some(AttrName::Header) if attr.value.is_some() => {
                    self.header = attr.lit_str_or_abort()?.value();
                }
                Some(AttrName::MaxWidth) => {
                    let val = attr.lit_int_or_abort()?;
                    self.max_width = val.base10_parse()?;

                    max_width_seen = true;
                }
                Some(AttrName::IsDefault) => match attr.value {
                    Some(_) => self.is_default = attr.lit_bool_or_abort()?.value,
                    None => self.is_default = true,
                },
                Some(AttrName::LeftAlign) => match attr.value {
                    Some(_) => self.left_align = attr.lit_bool_or_abort()?.value,
                    None => self.left_align = true,
                },
                Some(AttrName::SortBy) if attr.value.is_some() => {
                    self.sort_by = attr.lit_str_or_abort()?.value();
                }

                None | Some(AttrName::Header) | Some(AttrName::SortBy) => {}
            }
        }

        if !max_width_seen {
            panic!("max_width option is required but it is missing");
        }

        Ok(())
    }
}
