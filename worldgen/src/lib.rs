//! Generate [`Worlds`] using pre defined world
//! generation algorithms
//!
//! [`Worlds`]: ../myelin_environment/world/trait.World.html

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_doc,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use myelin_environment::Simulation;

pub mod generator;

/// API for [`World`] generation
///
/// [`World`]: ../myelin_environment/world/trait.World.html
pub trait WorldGenerator {
    /// Generate a new [`World`] and populates it with [`Objects`]
    ///
    /// [`World`]: ../myelin_environment/world/trait.World.html
    /// [`Objects`]: ../myelin_environment/object/struct.Body.html
    fn generate(&self) -> Box<dyn Simulation>;
}
