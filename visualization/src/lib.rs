//! In-browser visualization for myelin using a canvas with WASM

#![feature(box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown,
    clippy::unimplemented
)]

pub mod bootstrapper;
pub(crate) mod controller;
pub mod input_handler;
pub(crate) mod presenter;
pub mod view;
pub(crate) mod view_model;
