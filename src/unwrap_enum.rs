#![allow(dead_code)]

use proc_macro2::TokenStream;
use quasiquote::quasiquote;
use quote::{format_ident, ToTokens};
use syn::{Data, DeriveInput, Fields};

type Result<T = TokenStream> = syn::Result<T>;

pub fn expand_enum_is(input: &DeriveInput) -> Result {
    let Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "expected enum as derive input"
        ))
    };
    let mut impls = TokenStream::new();
    for variant in &data_enum.variants {
        let wildcard_pattern = match &variant.fields {
            Fields::Named(_) => quasiquote! { {..} },
            Fields::Unnamed(_) => quasiquote! { (..) },
            Fields::Unit => quasiquote! {},
        };
        quasiquote! {
            pub fn
            #{format_ident!("is_{}", variant.ident.to_string().to_lowercase())}
            (&self)
            -> bool
            {
                matches!{
                    self,
                    Self:: #{&variant.ident} #wildcard_pattern
                }
            }
        }
        .to_tokens(&mut impls);
    }
    Ok(expand_impl(input, &impls))
}

fn expand_impl(input: &DeriveInput, impls: &TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quasiquote! {
        impl #impl_generics #{&input.ident} #type_generics #where_clause {
            #impls
        }
    }
}
