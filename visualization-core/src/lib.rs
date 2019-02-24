//! Functionality shared by [`myelin_visualization_client`] and [`myelin_visualization_server`]
//!
//! [`myelin_visualization_client`]: ../myelin_visualization_client/index.html
//! [`myelin_visualization_server`]: ../myelin_visualization_server/index.html

#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

pub mod serialization;
pub mod view_model_delta;
