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

//! # unwrap-enum
//! A crate to generate methods to unwrap enums as certain variants, like `is_some`
//! and `is_none` on `Option`.
//!
//! # Example
//! ```rust
//! use unwrap_enum::{EnumAs, EnumIs};
//!
//! #[derive(Clone, Debug, EnumAs, EnumIs)]
//! enum Value {
//!     String(String),
//!     Int(i64)
//! }
//!
//! let value = Value::String("hello world".to_string());
//!
//! assert!(value.is_string());
//! assert!(!value.is_int());
//! assert!(matches!(value.as_string(), Some(string) if string == "hello world"));
//! assert!(matches!(value.as_int(), None));
//! ```
//!
//! # Todo
//! Implement EnumAsMut and EnumInto derive macros.

pub use unwrap_enum_proc_macro::{EnumAs, EnumIs};
