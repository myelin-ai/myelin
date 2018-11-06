//! Functionality shared by [`myelin_visualization_client`] and [`myelin_visualization_server`]

#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[macro_use]
extern crate serde_derive;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

pub mod serialization;
pub mod view_model_delta;
