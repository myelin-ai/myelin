#![feature(duration_float)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

#[macro_use]
extern crate serde_derive;

pub mod serialization;
pub mod transmission;
pub mod view_model_delta;
