//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints, duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod bootstrapper;
pub(crate) mod controller;
pub mod input_handler;
pub(crate) mod presenter;
pub(crate) mod serialize;
pub mod server;
pub(crate) mod transmitter;
pub mod view;
pub(crate) mod view_model;
