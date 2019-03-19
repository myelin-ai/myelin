//! Functionality shared by [`myelin_visualization_client`] and [`myelin_visualization_server`]
//!
//! [`myelin_visualization_client`]: ../myelin_visualization_client/index.html
//! [`myelin_visualization_server`]: ../myelin_visualization_server/index.html

#![feature(duration_float)]
#![warn(missing_docs, clippy::dbg_macro)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::unimplemented,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

pub mod serialization;
pub mod view_model_delta;
