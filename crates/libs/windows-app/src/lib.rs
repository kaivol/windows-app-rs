#![doc(html_no_source)]
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clashing_extern_declarations,
    unused_variables,
    dead_code,
    clippy::all
)]

extern crate windows;

#[cfg(feature = "bootstrap")]
pub mod bootstrap;

pub mod core;

#[cfg(feature = "DWriteCore")]
pub mod DWriteCore;
#[cfg(feature = "MRM")]
pub mod MRM;
#[cfg(feature = "Microsoft")]
pub mod Microsoft;
#[cfg(feature = "Windows")]
pub mod Windows;
