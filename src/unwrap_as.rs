use crate::common::{self, Ownership};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, Lifetime, Variant};

pub(crate) fn expand(target: &DataEnum) -> TokenStream {
    let mut tokens = TokenStream::new();
    for variant in target
        .variants
        .iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
    {
        tokens.extend(expand_method(variant));
    }
    tokens
}

fn expand_method(variant: &Variant) -> TokenStream {
    let lifetime = Lifetime::new("'unwrap_enum_lifetime", Span::call_site());
    let ownership = Ownership::Borrowed(Some(lifetime.clone()));
    let method_type = common::expand_fields_types_to_tuple(&variant.fields, &ownership);
    let method_name = format_ident!("as_{}", variant.ident.to_string().to_lowercase());
    let variant_ident = &variant.ident;
    let (destructure, bindings) = common::expand_destructure(&variant.fields);
    let bindings_expression = common::expand_destructuring_bindings(bindings.as_slice());
    quote! {
        pub fn #method_name < #lifetime > (& #lifetime self) -> Option< #method_type > {
            match self {
                Self:: #variant_ident #destructure => Some( #bindings_expression ),
                _ => None
            }
        }
    }
}
