use crate::serialization::{ViewModelDeserializer, ViewModelSerializer};
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::marker::PhantomData;

/// Provides methods for JSON serialization.
/// # Examples
/// ```
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
/// use myelin_visualization_core::serialization::{ViewModelSerializer, JsonSerializer};
///
/// let view_model_delta = ViewModelDelta::default();
/// let serializer = JsonSerializer::default();
/// let serialized = serializer.serialize_view_model_delta(&view_model_delta);
/// ```
#[derive(Debug, Default)]
pub struct JsonSerializer(PhantomData<()>);

/// Provides methods for JSON deserialization
/// # Examples
/// ```
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
/// use myelin_visualization_core::serialization::{ViewModelDeserializer, JsonDeserializer};
///
/// // Replace with a string that represents a ViewModelDelta
/// let source: Vec<u8> = r#"{}"#.into();
///
/// let deserializer = JsonDeserializer::default();
/// let deserialized = deserializer.deserialize_view_model_delta(&source);
/// ```
#[derive(Debug, Default)]
pub struct JsonDeserializer(PhantomData<()>);

impl ViewModelSerializer for JsonSerializer {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized = serde_json::to_string(view_model_delta)?;

        Ok(serialized.into())
    }
}

impl ViewModelDeserializer for JsonDeserializer {
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
    use myelin_geometry::*;

    #[test]
    fn serializes_full_delta() {
        let expected: Vec<u8> = r#"{"12":{"Updated":{"shape":{"vertices":[{"x":-5,"y":-5},{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"location":{"x":3,"y":4},"rotation":{"value":1.0},"mobility":{"Movable":{"x":2,"y":3}},"kind":"Organism","sensor":{"shape":{"vertices":[{"x":-10,"y":-12},{"x":10,"y":6},{"x":16,"y":0}]},"position":{"location":{"x":2,"y":3},"rotation":{"value":1.0}}}}}}"#.into();

        let object_description_delta = ObjectDescriptionDelta {
            kind: Some(Kind::Organism),
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
            sensor: Some(Some(Sensor {
                shape: PolygonBuilder::default()
                    .vertex(-10.0, -12.0)
                    .vertex(10.0, 6.0)
                    .vertex(16.0, 0.0)
                    .build()
                    .unwrap(),
                location: Point { x: 2.0, y: 3.0 },
                rotation: Radians::try_new(1.0).unwrap(),
            })),
        };

        let view_model_delta = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        print!(
            "{}",
            String::from_utf8(
                JsonSerializer::default()
                    .serialize_view_model_delta(&view_model_delta)
                    .unwrap()
            )
            .unwrap()
        );

        let serializer = JsonSerializer::default();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_works_with_empty_view_model() {
        let expected: Vec<u8> = r#"{}"#.into();

        let view_model_delta = ViewModelDelta::default();

        let serializer = JsonSerializer::default();
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
            sensor: Some(Some(Sensor {
                shape: PolygonBuilder::default()
                    .vertex(-10.0, -12.0)
                    .vertex(10.0, 6.0)
                    .vertex(16.0, 0.0)
                    .build()
                    .unwrap(),
                location: Point { x: 2.0, y: 3.0 },
                rotation: Radians::try_new(1.0).unwrap(),
            })),
        };

        let expected = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        let source: Vec<u8> = r#"{"12":{"Updated":{"shape":{"vertices":[{"x":-5,"y":-5},{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"location":{"x":3,"y":4},"rotation":{"value":1.0},"mobility":{"Movable":{"x":2,"y":3}},"kind":"Organism","sensor":{"shape":{"vertices":[{"x":-10,"y":-12},{"x":10,"y":6},{"x":16,"y":0}]},"position":{"location":{"x":2,"y":3},"rotation":{"value":1.0}}}}}}"#.into();

        print!(
            "{}",
            String::from_utf8(
                JsonSerializer::default()
                    .serialize_view_model_delta(&expected)
                    .unwrap()
            )
            .unwrap()
        );

        let deserializer = JsonDeserializer::default();
        let deserialized = deserializer.deserialize_view_model_delta(&source).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn deserialize_works_with_empty_view_model() {
        let expected = ViewModelDelta::default();

        let source: Vec<u8> = r#"{}"#.into();

        let deserializer = JsonDeserializer::default();
        let deserialized = deserializer.deserialize_view_model_delta(&source).unwrap();

        assert_eq!(expected, deserialized);
    }
}
