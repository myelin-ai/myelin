use crate::serialization::{ViewModelDeserializer, ViewModelSerializer};
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;

/// Provides methods for JSON serialization.
/// # Examples
/// ```
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
/// use myelin_visualization_core::serialization::{ViewModelSerializer, JsonSerializer};
///
/// let view_model_delta = ViewModelDelta::default();
/// let serializer = JsonSerializer::new();
/// let serialized = serializer.serialize_view_model_delta(&view_model_delta);
/// ```
#[derive(Debug)]
pub struct JsonSerializer;

/// Provides methods for JSON deserialization
/// # Examples
/// ```
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
/// use myelin_visualization_core::serialization::{ViewModelDeserializer, JsonDeserializer};
///
/// // Replace with a string that represents a ViewModelDelta
/// let source: Vec<u8> = r#"{}"#.into();
///
/// let deserializer = JsonDeserializer::new();
/// let deserialized = deserializer.deserialize_view_model_delta(&source);
/// ```
#[derive(Debug)]
pub struct JsonDeserializer;

impl JsonSerializer {
    /// Creates a new [`JsonSerializer`].
    pub fn new() -> Self {
        JsonSerializer
    }
}

impl JsonDeserializer {
    /// Creates a new [`JsonDeserializer`].
    pub fn new() -> Self {
        JsonDeserializer
    }
}

impl ViewModelSerializer for JsonSerializer {
    /// Serializes a `ViewModelDelta` to it's JSON string representation
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized = serde_json::to_string(view_model_delta)?;

        Ok(serialized.into())
    }
}

impl ViewModelDeserializer for JsonDeserializer {
    /// Deserializes a previously serialized `ViewModelDelta` from it's JSON string representation
    fn deserialize_view_model_delta(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>> {
        let json_string = String::from_utf8(buf.to_vec())?;
        let deserialized: ViewModelDelta = serde_json::from_str(&json_string)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::view_model_delta::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::PolygonBuilder;

    #[test]
    fn serializes_full_delta() {
        let expected: Vec<u8> = r#"{"12":{"type":"updated","shape":{"vertices":[{"x":-5,"y":-5},{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"location":{"x":3,"y":4},"rotation":{"value":1.0},"mobility":{"Movable":{"x":2,"y":3}},"kind":"Organism","sensor":{"shape":{"vertices":[{"x":-10,"y":-12},{"x":10,"y":6},{"x":16,"y":0}]},"position":{"location":{"x":2,"y":3},"rotation":{"value":1.0}}}}}"#.into();

        let object_description_delta = ObjectDescriptionDelta {
            kind: Some(Kind::Organism),
            shape: Some(
                PolygonBuilder::new()
                    .vertex(-5, -5)
                    .vertex(1, 1)
                    .vertex(2, 3)
                    .vertex(5, 6)
                    .build()
                    .unwrap(),
            ),
            mobility: Some(Mobility::Movable(Velocity { x: 2, y: 3 })),
            location: Some(Location { x: 3, y: 4 }),
            rotation: Some(Radians::new(1.0).unwrap()),
            sensor: Some(Some(Sensor {
                shape: PolygonBuilder::new()
                    .vertex(-10, -12)
                    .vertex(10, 6)
                    .vertex(16, 0)
                    .build()
                    .unwrap(),
                position: Position {
                    location: Location { x: 2, y: 3 },
                    rotation: Radians::new(1.0).unwrap(),
                },
            })),
        };

        let view_model_delta = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_works_with_empty_view_model() {
        let expected: Vec<u8> = r#"{}"#.into();

        let view_model_delta = ViewModelDelta::default();

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn deserializes_full_viewmodel() {
        let object_description_delta = ObjectDescriptionDelta {
            kind: Some(Kind::Organism),
            shape: Some(
                PolygonBuilder::new()
                    .vertex(-5, -5)
                    .vertex(1, 1)
                    .vertex(2, 3)
                    .vertex(5, 6)
                    .build()
                    .unwrap(),
            ),
            mobility: Some(Mobility::Movable(Velocity { x: 2, y: 3 })),
            location: Some(Location { x: 3, y: 4 }),
            rotation: Some(Radians::new(1.0).unwrap()),
            sensor: Some(Some(Sensor {
                shape: PolygonBuilder::new()
                    .vertex(-10, -12)
                    .vertex(10, 6)
                    .vertex(16, 0)
                    .build()
                    .unwrap(),
                position: Position {
                    location: Location { x: 2, y: 3 },
                    rotation: Radians::new(1.0).unwrap(),
                },
            })),
        };

        let expected = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        let source: Vec<u8> = r#"{"12":{"type":"updated","shape":{"vertices":[{"x":-5,"y":-5},{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"location":{"x":3,"y":4},"rotation":{"value":1.0},"mobility":{"Movable":{"x":2,"y":3}},"kind":"Organism","sensor":{"shape":{"vertices":[{"x":-10,"y":-12},{"x":10,"y":6},{"x":16,"y":0}]},"position":{"location":{"x":2,"y":3},"rotation":{"value":1.0}}}}}"#.into();

        print!("{}", String::from_utf8(JsonSerializer::new().serialize_view_model_delta(&expected).unwrap()).unwrap());

        let deserializer = JsonDeserializer::new();
        let deserialized = deserializer.deserialize_view_model_delta(&source).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn deserialize_works_with_empty_view_model() {
        let expected = ViewModelDelta::default();

        let source: Vec<u8> = r#"{}"#.into();

        let deserializer = JsonDeserializer::new();
        let deserialized = deserializer.deserialize_view_model_delta(&source).unwrap();

        assert_eq!(expected, deserialized);
    }
}
