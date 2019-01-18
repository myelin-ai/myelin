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
use std::fmt::Debug;

#[cfg(feature = "use-mocks")]
use mockiato::mockable;

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

/// Handles serialization for `AssociatedObjectData`
///
/// [`AssociatedObjectData`]: ./struct.AssociatedObjectData.html
#[cfg_attr(feature = "use-mocks", mockable)]
pub trait AssociatedObjectDataSerializer: Debug {
    /// Serialize associated object data
    fn serialize(
        &self,
        associated_object_data: &AssociatedObjectData,
    ) -> Vec<u8>;
}

/// Handles deserialization for `AssociatedObjectData`
///
/// [`AssociatedObjectData`]: ./struct.AssociatedObjectData.html
#[cfg_attr(feature = "use-mocks", mockable)]
pub trait AssociatedObjectDataDeserializer: Debug {
    /// Deserialize into associated object data
    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<AssociatedObjectData, Box<dyn Error>>;
}

/// Implements an `AssociatedObjectDataSerializer` using bincode
///
/// [`AssociatedObjectDataSerializer`]: ./trait.AssociatedObjectDataSerializer.html
#[derive(Debug, Default)]
pub struct AssociatedObjectDataBincodeSerializer {}

impl AssociatedObjectDataSerializer for AssociatedObjectDataBincodeSerializer {
    fn serialize(
        &self,
        associated_object_data: &AssociatedObjectData,
    ) -> Vec<u8> {
        bincode::serialize(associated_object_data)
            .expect("Unable to serialize associated object data")
    }
}

/// Implements an `AssociatedObjectDataDeserializer` using bincode
///
/// [`AssociatedObjectDataDeserializer`]: ./trait.AssociatedObjectDataDeserializer.html
#[derive(Debug, Default)]
pub struct AssociatedObjectDataBincodeDeserializer {}

impl AssociatedObjectDataDeserializer for AssociatedObjectDataBincodeDeserializer {
    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<AssociatedObjectData, Box<dyn Error>> {
        bincode::deserialize(data).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize_work() {
        let serializer = AssociatedObjectDataBincodeSerializer::default();
        let deserializer = AssociatedObjectDataBincodeDeserializer::default();

        let associated_object_data = AssociatedObjectData {
            name: Some(String::from("Foo")),
            kind: Kind::Plant,
        };

        let serialized_data = serializer.serialize(&associated_object_data);
        let deserialized_associated_object_data = deserializer
            .deserialize(&serialized_data)
            .expect("Unable to deserialize data");

        assert_eq!(associated_object_data, deserialized_associated_object_data);
    }

    #[test]
    #[should_panic]
    fn deserialize_fails_with_invalid_data() {
        let deserializer = AssociatedObjectDataBincodeDeserializer::default();

        let invalid_data = String::from("banana").into_bytes();
        deserializer
            .deserialize(&invalid_data)
            .unwrap();
    }

}
