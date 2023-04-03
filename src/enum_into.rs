use crate::common;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, Variant};

pub fn expand(target: &DataEnum) -> TokenStream {
    let mut tokens = TokenStream::new();
    for variant in target
        .variants
        .iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
    {
        tokens.extend(expand_method(variant))
    }
    tokens
}

pub fn expand_method(variant: &Variant) -> TokenStream {
    let method_name = format_ident!("into_{}", variant.ident.to_string().to_lowercase());
    let method_type =
        common::expand_fields_types_to_tuple(&variant.fields, common::Ownership::Owned);
    let variant_ident = &variant.ident;
    let (destructure, bindings) = common::expand_destructure(&variant.fields);
    let bindings_expression = common::expand_destructuring_bindings(bindings.as_slice());
    quote! {
        pub fn #method_name (self) -> ::std::option::Option< #method_type > {
            match self {
                Self:: #variant_ident #destructure => ::std::option::Option::Some( #bindings_expression ),
                _ => ::std::option::Option::None
            }
        }
    }
}
