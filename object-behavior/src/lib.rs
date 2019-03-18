//! Implementation of the behaviours and interactions between
//! objects that can be placed in a simulation

#![feature(specialization)]
#![warn(missing_docs)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::unimplemented,
    clippy::default_trait_access,
    clippy::enum_glob_use
)]

// Not named "static" because that would be a keyword
mod static_behavior;
pub use self::static_behavior::Static;

pub mod organism;
pub mod stochastic_spreading;
