//! Implementation of the behaviours and interactions between
//! objects that can be placed in a simulation

#![feature(tool_lints)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

pub mod organism;
pub mod plant;
pub mod terrain;
pub mod water;
