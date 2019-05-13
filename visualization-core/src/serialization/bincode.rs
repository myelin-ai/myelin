//! Implementation of [`ViewModelSerializer`] and [`ViewModelDeserializer`] using
//! [`bincode`], a compact binary encoding format.

use crate::serialization::{ViewModelDeserializer, ViewModelSerializer};
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::marker::PhantomData;
use wonderbox::autoresolvable;

/// Provides methods for serialization using using
/// [`bincode`], a compact binary encoding format.
///
/// # Examples
/// ```
/// use myelin_visualization_core::serialization::{BincodeSerializer, ViewModelSerializer};
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
///
/// let view_model_delta = ViewModelDelta::default();
/// let serializer = BincodeSerializer::default();
/// let serialized = serializer.serialize_view_model_delta(&view_model_delta);
/// ```
///
/// [`bincode`]: https://github.com/TyOverby/bincode
#[derive(Debug, Default)]
pub struct BincodeSerializer(PhantomData<()>);

#[autoresolvable]
impl BincodeSerializer {
    /// Returns a new [`BincodeSerializer`]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ViewModelSerializer for BincodeSerializer {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(bincode::serialize(view_model_delta)?)
    }
}

/// Provides methods for deserialization using using
/// [`bincode`], a compact binary encoding format.
/// # Examples
/// ```
/// use myelin_visualization_core::serialization::{BincodeDeserializer, ViewModelDeserializer};
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
///
/// // Replace with a `Vec` that represents a ViewModelDelta
/// let source: Vec<u8> = vec![];
///
/// let deserializer = BincodeDeserializer::default();
/// let deserialized = deserializer.deserialize_view_model_delta(&source);
/// ```
///
/// [`bincode`]: https://github.com/TyOverby/bincode
#[derive(Debug, Default)]
pub struct BincodeDeserializer(PhantomData<()>);

#[autoresolvable]
impl BincodeDeserializer {
    /// Returns a new [`BincodeDeserializer`]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ViewModelDeserializer for BincodeDeserializer {
    fn deserialize_view_model_delta(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>> {
        Ok(bincode::deserialize(buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_model_delta::*;
    use maplit::hashmap;
    use myelin_engine::geometry::*;
    use myelin_engine::object::*;
    use myelin_object_data::{AdditionalObjectDescription, Kind};

    #[test]
    fn serializes_full_delta() {
        let object_description_delta = ObjectDescriptionDelta {
            shape: Some(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(1.0, 1.0)
                    .vertex(2.0, 3.0)
                    .vertex(5.0, 6.0)
                    .build()
                    .unwrap(),
            ),
            mobility: Some(Mobility::Movable(Vector { x: 2.0, y: 3.0 })),
            location: Some(Point { x: 3.0, y: 4.0 }),
            rotation: Some(Radians::try_new(1.0).unwrap()),
            associated_data: Some(associated_data()),
        };

        let view_model_delta = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        let serializer = BincodeSerializer::default();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert!(!serialized.is_empty());
        const MAX_SIZE: usize = 1024;
        assert!(serialized.len() < MAX_SIZE);
    }

    #[test]
    fn serialize_works_with_empty_view_model() {
        let view_model_delta = ViewModelDelta::default();

        let serializer = BincodeSerializer::default();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert!(!serialized.is_empty());
        const MAX_SIZE: usize = 1024;
        assert!(serialized.len() < MAX_SIZE);
    }

    #[test]
    fn deserializes_full_viewmodel() {
        let object_description_delta = ObjectDescriptionDelta {
            shape: Some(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(1.0, 1.0)
                    .vertex(2.0, 3.0)
                    .vertex(5.0, 6.0)
                    .build()
                    .unwrap(),
            ),
            mobility: Some(Mobility::Movable(Vector { x: 2.0, y: 3.0 })),
            location: Some(Point { x: 3.0, y: 4.0 }),
            rotation: Some(Radians::try_new(1.0).unwrap()),
            associated_data: Some(associated_data()),
        };

        let expected = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };
        let serialized = BincodeSerializer::default()
            .serialize_view_model_delta(&expected)
            .unwrap();

        let deserializer = BincodeDeserializer::default();
        let deserialized = deserializer
            .deserialize_view_model_delta(&serialized)
            .unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn deserialize_works_with_empty_view_model() {
        let expected = ViewModelDelta::default();
        let serialized = BincodeSerializer::default()
            .serialize_view_model_delta(&expected)
            .unwrap();

        let deserializer = BincodeDeserializer::default();
        let deserialized = deserializer
            .deserialize_view_model_delta(&serialized)
            .unwrap();

        assert_eq!(expected, deserialized);
    }

    fn associated_data() -> AdditionalObjectDescription {
        AdditionalObjectDescription {
            name: Some(String::from("Cat")),
            height: 1.5,
            kind: Kind::Organism,
        }
    }
}
