//! In-browser visualization for myelin using a canvas with WASM

#![feature(duration_float, fnbox, box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    intra_doc_link_resolution_failure,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate nameof;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

#[macro_use]
mod fixed_interval_sleeper;

mod client;
mod connection;
mod connection_acceptor;
mod constant;
mod controller;
mod presenter;
mod server;

pub use self::server::start_server;
