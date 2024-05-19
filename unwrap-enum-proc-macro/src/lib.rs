// Copyright (C) 2024 John Turner

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![deny(clippy::pedantic, clippy::use_self)]
#![allow(clippy::missing_panics_doc)]

mod unwrap_enum;

#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match unwrap_enum::expand_enum_is(&parsed_input) {
        Ok(output) => output.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(EnumAs)]
pub fn enum_as(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = syn::parse_macro_input!(input as syn::DeriveInput);
    match unwrap_enum::expand_enum_as(&parsed_input, false) {
        Ok(output) => output.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[allow(unused_variables)]
#[proc_macro_derive(EnumAsMut)]
pub fn enum_as_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[allow(unused_variables)]
#[proc_macro_derive(EnumInto)]
pub fn enum_into(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}
