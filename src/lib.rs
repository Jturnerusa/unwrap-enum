#![allow(dead_code)]

mod unwrap_is;

#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let methods = unwrap_is::expand(&common::input_as_enum(&input));
    let expanded_impl = common::expand_impl(input, methods);
    expanded_impl.into()
}

pub(crate) mod common {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Data, DataEnum, DeriveInput, Fields};

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

    pub fn input_as_enum(input: &DeriveInput) -> DataEnum {
        match &input.data {
            Data::Enum(e) => e.clone(),
            _ => panic!("derive input is not an enum"),
        }
    }

    pub fn expand_wildcard(fields: Fields) -> TokenStream {
        match fields {
            Fields::Unnamed(_) => quote! { (..) },
            Fields::Named(_) => quote! { {..} },
            Fields::Unit => quote! {},
        }
    }

    pub fn expand_destructure(fields: Fields) -> TokenStream {
        match fields {
            Fields::Unnamed(_) => {
                let bindings = (0..fields.len()).map(|i| format!("unwrap_enum_binding_{i}"));
                quote! {
                    (#(#bindings),*)
                }
            }
            Fields::Named(named_fields) => {
                let bindings = named_fields.named.iter().cloned().map(|f| f.ident.unwrap());
                quote! {
                    {#(#bindings),*}
                }
            }
            _ => panic!("Can't destructure a unit variant"),
        }
    }

    pub fn expand_method_return_types(fields: Fields) -> TokenStream {
        let types = fields.iter().cloned().map(|f| f.ty);
        quote! {
            (#(#types),*)
        }
    }
}
