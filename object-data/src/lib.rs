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
    clippy::explicit_into_iter_loop
)]
#![feature(box_syntax)]
#![feature(specialization)]

use myelin_clone_box::clone_box;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Debug, Display};

#[cfg(feature = "use-mocks")]
use mockiato::mockable;

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

/// Handles serialization for `AdditionalObjectDescription`
///
/// [`AdditionalObjectDescription`]: ./struct.AdditionalObjectDescription.html
#[cfg_attr(feature = "use-mocks", mockable)]
pub trait AdditionalObjectDescriptionSerializer: Debug {
    /// Serialize associated object data
    fn serialize(&self, additional_object_description: &AdditionalObjectDescription) -> Vec<u8>;
}

/// Handles deserialization for `AdditionalObjectDescription`
///
/// [`AdditionalObjectDescription`]: ./struct.AdditionalObjectDescription.html
#[cfg_attr(feature = "use-mocks", mockable(static_references))]
pub trait AdditionalObjectDescriptionDeserializer:
    Debug + AdditionalObjectDescriptionDeserializerClone
{
    /// Deserialize into associated object data
    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<AdditionalObjectDescription, AdditionalObjectDescriptionDeserializerError>;
}

clone_box!(
    AdditionalObjectDescriptionDeserializer,
    AdditionalObjectDescriptionDeserializerClone
);

/// Implements an `AdditionalObjectDescriptionSerializer` using bincode
///
/// [`AdditionalObjectDescriptionSerializer`]: ./trait.AdditionalObjectDescriptionSerializer.html
#[derive(Debug, Default, Clone)]
pub struct AdditionalObjectDescriptionBincodeSerializer {}

impl AdditionalObjectDescriptionSerializer for AdditionalObjectDescriptionBincodeSerializer {
    fn serialize(&self, additional_object_description: &AdditionalObjectDescription) -> Vec<u8> {
        bincode::serialize(additional_object_description)
            .expect("Unable to serialize associated object data")
    }
}

/// Implements an `AdditionalObjectDescriptionDeserializer` using bincode
///
/// [`AdditionalObjectDescriptionDeserializer`]: ./trait.AdditionalObjectDescriptionDeserializer.html
#[derive(Debug, Default, Clone)]
pub struct AdditionalObjectDescriptionBincodeDeserializer {}

impl AdditionalObjectDescriptionDeserializer for AdditionalObjectDescriptionBincodeDeserializer {
    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<AdditionalObjectDescription, AdditionalObjectDescriptionDeserializerError> {
        bincode::deserialize(data).map_err(|err| AdditionalObjectDescriptionDeserializerError {
            message: err.description().to_string(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Something unexpected happened when trying to deserialize [`AdditionalObjectDescription`]
pub struct AdditionalObjectDescriptionDeserializerError {
    message: String,
}

impl Display for AdditionalObjectDescriptionDeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AdditionalObjectDescriptionDeserializerError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize_work() {
        let serializer = AdditionalObjectDescriptionBincodeSerializer::default();
        let deserializer = AdditionalObjectDescriptionBincodeDeserializer::default();

        let additional_object_description = AdditionalObjectDescription {
            name: Some(String::from("Foo")),
            kind: Kind::Plant,
            height: 1.8,
        };

        let serialized_data = serializer.serialize(&additional_object_description);
        let deserialized_additional_object_description = deserializer
            .deserialize(&serialized_data)
            .expect("Unable to deserialize data");

        assert_eq!(
            additional_object_description,
            deserialized_additional_object_description
        );
    }

    #[test]
    fn deserialize_fails_with_invalid_data() {
        let deserializer = AdditionalObjectDescriptionBincodeDeserializer::default();

        let invalid_data = String::from("banana").into_bytes();
        assert!(deserializer.deserialize(&invalid_data).is_err());
    }

}
