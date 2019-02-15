//! Implementation of the behaviours and interactions between
//! objects that can be placed in a simulation

#![feature(specialization)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate maplit;

// Not named "static" because that would be a keyword
mod static_behavior;
pub use self::static_behavior::Static;

pub mod stochastic_spreading;
pub mod organism;
