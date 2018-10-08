//! In-browser visualization for myelin using a canvas with WASM

#![feature(tool_lints, duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

#[macro_use]
extern crate log;

mod connection;
mod constant;
mod controller;
mod presenter;
mod serialize;
mod server;
mod snapshot;
mod transmitter;

pub use self::server::start_server;
