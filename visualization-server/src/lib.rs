//! In-browser visualization for myelin using a canvas with WASM

#![feature(duration_float)]
#![feature(fnbox)]
#![feature(box_syntax)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented,
    clippy::default_trait_access,
    clippy::enum_glob_use
)]

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
