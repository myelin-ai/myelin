//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints, duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

pub mod bootstrapper;
pub(crate) mod deserialize;
pub mod input_handler;
pub mod view;
