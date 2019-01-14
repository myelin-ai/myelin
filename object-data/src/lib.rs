//! Definition of associated object data (mainly used within visualization)

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use serde_derive::{Deserialize, Serialize};
use std::error::Error;

/// The data associated with an object
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
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
pub fn serialize_associated_object_data(associated_object_data: &AssociatedObjectData) -> Vec<u8> {
    bincode::serialize(associated_object_data).expect("Unable to serialize associated object data")
}

/// Deserialize into associated object data
pub fn deserialize_associated_object_data(data: &[u8]) -> Result<AssociatedObjectData, Box<dyn Error>> {
    bincode::deserialize(data).map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize_work() {
        let associated_object_data = AssociatedObjectData {
            name: Some(String::from("Foo")),
            kind: Kind::Plant,
        };

        let serialized_data = serialize_associated_object_data(&associated_object_data);
        let deserialized_associated_object_data = deserialize_associated_object_data(&serialized_data).expect("Unable to deserialize data");

        assert_eq!(associated_object_data, deserialized_associated_object_data);
    }
}
