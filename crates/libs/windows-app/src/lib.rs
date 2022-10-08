#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clashing_extern_declarations, unused_variables, dead_code, clippy::all)]

extern crate windows;

pub mod core;

mod Microsoft;
pub use Microsoft::*;

#[cfg(feature = "bootstrap")]
pub mod bootstrap;
