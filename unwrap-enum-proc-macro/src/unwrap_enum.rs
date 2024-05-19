#![allow(dead_code)]

use proc_macro2::TokenStream;
use quasiquote::quasiquote;
use quote::{format_ident, ToTokens};
use syn::{Data, DeriveInput, Fields, Ident, Variant};

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

pub fn expand_enum_as(input: &DeriveInput, is_mut: bool) -> Result {
    let Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "expected enum as derive input"
        ))
    };
    let mut output = TokenStream::new();
    for variant in data_enum
        .variants
        .iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
    {
        let (bindings, pattern) = destructure_variant(variant);
        let tuple_expr = expand_tuple_expr(bindings.iter());
        let method_type = {
            let mut iter = variant.fields.iter().map(|field| &field.ty);
            let count = iter.clone().count();
            if count < 1 {
                quasiquote! { #{&iter.next().unwrap()} }
            } else {
                quasiquote! { (#(&#iter),*) }
            }
        };
        let method_name = if is_mut {
            format_ident!("as_mut_{}", variant.ident.to_string().to_lowercase())
        } else {
            format_ident!("as_{}", variant.ident.to_string().to_lowercase())
        };
        quasiquote! {
            pub fn
            #method_name
            ( & #{is_mut.then(|| quasiquote! { mut })} self )
            -> ::std::option::Option< #method_type >
            {
                match self {
                    Self:: #{&variant.ident}  #pattern => ::std::option::Option::Some ( #tuple_expr ),
                    _ => None
                }
            }
        }
        .to_tokens(&mut output);
    }
    Ok(expand_impl(input, &output))
}

fn expand_tuple_expr<'a>(mut idents: impl Iterator<Item = &'a Ident> + Clone) -> TokenStream {
    if idents.clone().count() > 1 {
        quasiquote! { ( #(#idents),* ) }
    } else {
        quasiquote! { #{idents.next().unwrap()} }
    }
}

fn destructure_variant(variant: &Variant) -> (Vec<Ident>, TokenStream) {
    let bindings = match &variant.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| f.ident.as_ref().cloned().unwrap())
            .collect::<Vec<Ident>>(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| format_ident!("field_{i}"))
            .collect::<Vec<Ident>>(),
        Fields::Unit => unreachable!(),
    };
    let pattern = match &variant.fields {
        Fields::Named(_) => quasiquote! { { #(#bindings),* } },
        Fields::Unnamed(_) => quasiquote! { ( #(#bindings),* ) },
        Fields::Unit => unreachable!(),
    };
    (bindings, pattern)
}

fn expand_impl(input: &DeriveInput, impls: &TokenStream) -> TokenStream {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    quasiquote! {
        impl #impl_generics #{&input.ident} #type_generics #where_clause {
            #impls
        }
    }
}
