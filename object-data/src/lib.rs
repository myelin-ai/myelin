//! Definition of associated object data (mainly used within visualization)

#![warn(missing_docs, clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop,
    clippy::wrong_self_convention
)]

use serde::{Deserialize, Serialize};

/// The behaviourless description of an object that has
/// been placed inside a [`Simulation`].
///
/// [`Simulation`]: myelin_engine::simulation::Simulation
pub type ObjectDescription = myelin_engine::object::ObjectDescription<AdditionalObjectDescription>;

/// An object that is stored in the simulation
pub type Object<'a> = myelin_engine::object::Object<'a, AdditionalObjectDescription>;

/// The data associated with an object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalObjectDescription {
    /// The name of an object
    pub name: Option<String>,

    /// The kind of an object
    pub kind: Kind,

    /// The object's height in meters
    pub height: f64,
}

/// The part of an object that is responsible for custom
/// behavior and interactions
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Kind {
    /// An intelligent organism featuring a neural network
    Organism,
    /// A self-spreading plant, ready for consumption
    Plant,
    /// A stationary body of water
    Water,
    /// Impassable terrain
    Terrain,
}
