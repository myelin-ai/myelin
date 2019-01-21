//! In-browser visualization for myelin using a canvas with WASM

#![feature(box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    intra_doc_link_resolution_failure,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate nameof;

pub use self::bootstrapper::*;
pub use self::input_handler::*;

mod bootstrapper;
mod controller;
mod input_handler;
mod presenter;
mod view;
mod view_model;
