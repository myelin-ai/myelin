//! Generate [`Worlds`] using pre defined world
//! generation algorithms
//!
//! [`Worlds`]: ../myelin_environment/world/trait.World.html

#![deny(rust_2018_idioms)]

use myelin_environment::world::World;

pub mod generator;

/// API for [`World`] generation
///
/// [`World`]: ../myelin_environment/world/trait.World.html
pub trait WorldGenerator {
    /// Generate a new [`World`] and populate it with [`Objects`]
    ///
    /// [`World`]: ../myelin_environment/world/trait.World.html
    /// [`Objects`]: ../myelin_environment/object/struct.LocalObject.html
    fn generate(&self) -> Box<dyn World>;
}
