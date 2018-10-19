//! In-browser visualization for myelin using a canvas with WASM

#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

pub mod bootstrapper;
pub mod controller;
pub(crate) mod deserialize;
pub mod input_handler;
pub mod presenter;
pub mod view;
pub mod view_model;
