//! Implementation of the behaviours and interactions between
//! objects that can be placed in a simulation

#![feature(tool_lints, specialization)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown
)]

// Not named "static" because that would be a keyword
mod static_behavior;
pub use self::static_behavior::Static;

pub mod stochastic_spreading;
