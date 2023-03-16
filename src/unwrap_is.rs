use proc_macro2::TokenStream;

use syn::{DataEnum, Variant};

use quote::{format_ident, quote};

use crate::common::expand_wildcard;

pub(crate) fn expand(target: &DataEnum) -> TokenStream {
    let mut methods = TokenStream::new();
    for variant in &target.variants {
        let m = expand_method(variant);
        methods.extend(m);
    }
    methods
}

fn expand_method(variant: &Variant) -> TokenStream {
    let variant_ident = &variant.ident;
    let method_name = format_ident!("is_{}", variant_ident.to_string().to_ascii_lowercase());
    let wildcard = expand_wildcard(variant.clone().fields);
    quote! {
        pub fn #method_name (&self) -> bool {
            matches!(self, Self::#variant_ident #wildcard)
        }
    }
}
