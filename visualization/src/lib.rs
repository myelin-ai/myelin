//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints, duration_as_u128)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

pub mod benchmark;
pub(crate) mod benchmark_utils;
pub mod bootstrapper;
pub(crate) mod controller;
pub mod input_handler;
pub(crate) mod presenter;
pub mod view;
pub(crate) mod view_model;
