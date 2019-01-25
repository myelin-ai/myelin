//! Definition of associated object data (mainly used within visualization)

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    intra_doc_link_resolution_failure,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

#[cfg(feature = "use-mocks")]
use mockiato::mockable;

/// The data associated with an object
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdditionalObjectDescription {
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

/// Handles serialization for `AdditionalObjectDescription`
///
/// [`AdditionalObjectDescription`]: ./struct.AdditionalObjectDescription.html
#[cfg_attr(feature = "use-mocks", mockable)]
pub trait AdditionalObjectDescriptionSerializer: Debug {
    /// Serialize associated object data
    fn serialize(&self, associated_object_data: &AdditionalObjectDescription) -> Vec<u8>;
}

/// Handles deserialization for `AdditionalObjectDescription`
///
/// [`AdditionalObjectDescription`]: ./struct.AdditionalObjectDescription.html
#[cfg_attr(feature = "use-mocks", mockable)]
pub trait AdditionalObjectDescriptionDeserializer: Debug {
    /// Deserialize into associated object data
    fn deserialize(&self, data: &[u8]) -> Result<AdditionalObjectDescription, Box<dyn Error>>;
}

/// Implements an `AdditionalObjectDescriptionSerializer` using bincode
///
/// [`AdditionalObjectDescriptionSerializer`]: ./trait.AdditionalObjectDescriptionSerializer.html
#[derive(Debug, Default)]
pub struct AdditionalObjectDescriptionBincodeSerializer {}

impl AdditionalObjectDescriptionSerializer for AdditionalObjectDescriptionBincodeSerializer {
    fn serialize(&self, associated_object_data: &AdditionalObjectDescription) -> Vec<u8> {
        bincode::serialize(associated_object_data)
            .expect("Unable to serialize associated object data")
    }
}

/// Implements an `AdditionalObjectDescriptionDeserializer` using bincode
///
/// [`AdditionalObjectDescriptionDeserializer`]: ./trait.AdditionalObjectDescriptionDeserializer.html
#[derive(Debug, Default)]
pub struct AdditionalObjectDescriptionBincodeDeserializer {}

impl AdditionalObjectDescriptionDeserializer for AdditionalObjectDescriptionBincodeDeserializer {
    fn deserialize(&self, data: &[u8]) -> Result<AdditionalObjectDescription, Box<dyn Error>> {
        bincode::deserialize(data).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize_work() {
        let serializer = AdditionalObjectDescriptionBincodeSerializer::default();
        let deserializer = AdditionalObjectDescriptionBincodeDeserializer::default();

        let associated_object_data = AdditionalObjectDescription {
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
    fn deserialize_fails_with_invalid_data() {
        let deserializer = AdditionalObjectDescriptionBincodeDeserializer::default();

        let invalid_data = String::from("banana").into_bytes();
        assert!(deserializer.deserialize(&invalid_data).is_err());
    }

}
