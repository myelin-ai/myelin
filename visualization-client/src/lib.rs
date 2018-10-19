//! In-browser visualization for myelin using a canvas with WASM

#![feature(box_syntax)]
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
mod controller;
pub mod input_handler;
mod presenter;
mod view;
mod view_model;
