//! Definition of associated object data (mainly used within visualization)

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use serde_derive::{Deserialize, Serialize};

/// The data associated with an object
#[derive(Debug, Serialize, Deserialize)]
pub struct AssociatedObjectData {
    /// The name of an object
    pub name: Option<String>,

    /// The kind of an object
    pub kind: Kind,
}

/// The part of an object that is responsible for custom
/// behavior and interactions
#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

/// Serialize associated object data
pub fn serialize(associated_object_data: &AssociatedObjectData) -> Vec<u8> {
    bincode::serialize(associated_object_data)
}

/// Deserialize into associated object data
pub fn deserialize(data: &[u8]) -> AssociatedObjectData {
    bincode::deserialize(data).expect("Unable to deserialize associated data")
}
