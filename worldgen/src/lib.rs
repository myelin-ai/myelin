//! Generate [`Worlds`] using pre defined world
//! generation algorithms
//!
//! [`Worlds`]: ../myelin_engine/world/trait.World.html

#![feature(box_syntax)]
#![warn(missing_docs, clippy::dbg_macro)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::unimplemented,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

pub use self::generator::*;
pub use self::name_provider::*;
use myelin_engine::prelude::*;

mod generator;
mod name_provider;

/// API for [`World`] generation
///
/// [`World`]: ../myelin_engine/world/trait.World.html
pub trait WorldGenerator {
    /// Generate a new [`World`] and populates it with [`Objects`]
    ///
    /// [`World`]: ../myelin_engine/world/trait.World.html
    /// [`Objects`]: ../myelin_engine/object/struct.Body.html
    fn generate(&mut self) -> Box<dyn Simulation>;
}
