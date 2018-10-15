//! In-browser visualization for myelin using a canvas with WASM

#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

pub mod bootstrapper;
mod controller;
pub mod input_handler;
mod presenter;
mod view;
mod view_model;
