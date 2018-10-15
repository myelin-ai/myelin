//! In-browser visualization for myelin using a canvas with WASM

#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

#[macro_use]
extern crate log;

mod client;
mod connection;
mod constant;
mod controller;
mod presenter;
mod server;
mod snapshot;

pub use self::server::start_server;
