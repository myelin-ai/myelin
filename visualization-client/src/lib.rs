//! In-browser visualization for myelin using a canvas with WASM

#![feature(box_syntax)]
#![warn(missing_docs, clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

pub use self::bootstrapper::*;
pub use self::input_handler::*;

mod bootstrapper;
mod controller;
mod input_handler;
mod presenter;
mod view;
mod view_model;
