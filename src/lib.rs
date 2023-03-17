#![allow(dead_code)]

mod enum_as;
mod enum_is;

#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let methods = enum_is::expand(common::input_as_enum(&input));
    let expanded_impl = common::expand_impl(input, methods);
    expanded_impl.into()
}

#[proc_macro_derive(EnumAs)]
pub fn enum_as(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let methods = enum_as::expand(common::input_as_enum(&input), false);
    let expanded_impl = common::expand_impl(input, methods);
    expanded_impl.into()
}

#[proc_macro_derive(EnumAsMut)]
pub fn enum_as_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let methods = enum_as::expand(common::input_as_enum(&input), true);
    let expanded_impl = common::expand_impl(input, methods);
    expanded_impl.into()
}

pub(crate) mod common {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Lifetime, Type};

    pub enum Ownership<'a> {
        Owned,
        Borrowed(Option<&'a Lifetime>),
        MutBorrowed(Option<&'a Lifetime>),
    }

    pub fn expand_impl(input: DeriveInput, methods: TokenStream) -> TokenStream {
        let t = input.ident;
        let (impl_generics, type_generics, where_generics) = input.generics.split_for_impl();
        quote! {
            impl #impl_generics #t #type_generics
            #where_generics
            {
                #methods
            }
        }
    }

    pub fn input_as_enum(input: &DeriveInput) -> &DataEnum {
        match &input.data {
            Data::Enum(e) => e,
            _ => panic!("derive input is not an enum"),
        }
    }

    pub fn expand_wildcard(fields: &Fields) -> TokenStream {
        match fields {
            Fields::Unnamed(_) => quote! { (..) },
            Fields::Named(_) => quote! { {..} },
            Fields::Unit => quote! {},
        }
    }

    pub fn expand_destructure(fields: &Fields) -> (TokenStream, Vec<Ident>) {
        match fields {
            Fields::Unnamed(_) => {
                let bindings: Vec<Ident> = (0..fields.len())
                    .map(|i| format_ident!("unwrap_enum_binding_{}", i))
                    .collect();
                let tokens = quote! {
                    (#(#bindings),*)
                };
                (tokens, bindings)
            }
            Fields::Named(named_fields) => {
                let bindings: Vec<Ident> = named_fields
                    .named
                    .iter()
                    .cloned()
                    .map(|f| f.ident.unwrap())
                    .collect();
                let tokens = quote! {
                    {#(#bindings),*}
                };
                (tokens, bindings)
            }
            _ => panic!("Can't destructure a unit variant"),
        }
    }

    pub fn expand_fields_types_to_tuple(fields: &Fields, ownership: &Ownership) -> TokenStream {
        let ownership_tokens = match ownership {
            Ownership::Owned => quote! {},
            Ownership::Borrowed(lifetime) => quote! { & #lifetime },
            Ownership::MutBorrowed(lifetime) => quote! { & #lifetime mut },
        };
        let types: Vec<&Type> = fields.iter().map(|f| &f.ty).collect();
        if types.len() == 1 {
            let t = types[0];
            quote! {
                #ownership_tokens #t
            }
        } else {
            quote! {
                (#(#ownership_tokens #types),*)
            }
        }
    }

    pub fn expand_destructuring_bindings(bindings: &[Ident]) -> TokenStream {
        if bindings.len() == 1 {
            let b = &bindings[0];
            quote! {
                #b
            }
        } else {
            quote! {
                (#(#bindings),*)
            }
        }
    }
}
