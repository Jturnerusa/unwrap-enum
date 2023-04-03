use crate::common::{self, Ownership};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, Lifetime, Variant};

pub(crate) fn expand(target: &DataEnum, mutable: bool) -> TokenStream {
    let mut tokens = TokenStream::new();
    for variant in target
        .variants
        .iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
    {
        tokens.extend(expand_method(variant, mutable));
    }
    tokens
}

fn expand_method(variant: &Variant, mutable: bool) -> TokenStream {
    let lifetime = Lifetime::new("'unwrap_enum_lifetime", Span::call_site());
    let variant_ident = &variant.ident;
    let (destructure, bindings) = common::expand_destructure(&variant.fields);
    let bindings_expression = common::expand_destructuring_bindings(bindings.as_slice());
    let (method_name, method_type, mut_token) = if mutable {
        (
            format_ident!("as_mut_{}", variant.ident.to_string().to_lowercase()),
            common::expand_fields_types_to_tuple(
                &variant.fields,
                &Ownership::MutBorrowed(Some(&lifetime)),
            ),
            quote! { mut },
        )
    } else {
        (
            format_ident!("as_{}", variant.ident.to_string().to_lowercase()),
            common::expand_fields_types_to_tuple(
                &variant.fields,
                &Ownership::Borrowed(Some(&lifetime)),
            ),
            quote! {},
        )
    };
    quote! {
        pub fn #method_name < #lifetime > (& #lifetime #mut_token self  )
        -> ::std::option::Option< #method_type >
        {
            match self {
                Self:: #variant_ident #destructure => ::std::option::Option::Some( #bindings_expression ),
                _ => None
            }
        }
    }
}
