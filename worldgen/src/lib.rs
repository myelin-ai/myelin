//! Generate [`Worlds`] using pre defined world
//! generation algorithms
//!
//! [`Worlds`]: ../myelin_environment/world/trait.World.html

#![feature(box_syntax)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    intra_doc_link_resolution_failure,
    clippy::doc_markdown,
    clippy::unimplemented
)]

#[macro_use]
extern crate nameof;

pub use self::generator::*;
pub use self::name_provider::*;
use myelin_environment::prelude::*;

mod generator;
mod name_provider;

/// API for [`World`] generation
///
/// [`World`]: ../myelin_environment/world/trait.World.html
pub trait WorldGenerator {
    /// Generate a new [`World`] and populates it with [`Objects`]
    ///
    /// [`World`]: ../myelin_environment/world/trait.World.html
    /// [`Objects`]: ../myelin_environment/object/struct.Body.html
    fn generate(&mut self) -> Box<dyn Simulation>;
}
