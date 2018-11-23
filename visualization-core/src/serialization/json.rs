use crate::serialization::{ViewModelDeserializer, ViewModelSerializer};
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::marker::PhantomData;

/// Provides methods for JSON serialization.
/// # Examples
/// ```
/// use myelin_visualization_core::serialization::{JsonSerializer, ViewModelSerializer};
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
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
/// use myelin_visualization_core::serialization::{JsonDeserializer, ViewModelDeserializer};
/// use myelin_visualization_core::view_model_delta::ViewModelDelta;
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

    const EXPECTED_JSON: &str = r#"{"12":{"Updated":{"shape":{"vertices":[{"x":-5.0,"y":-5.0},{"x":1.0,"y":1.0},{"x":2.0,"y":3.0},{"x":5.0,"y":6.0}]},"location":{"x":3.0,"y":4.0},"rotation":{"value":1.0},"mobility":{"Movable":{"x":2.0,"y":3.0}},"kind":"Organism"}}}"#;

    #[test]
    fn serializes_full_delta() {
        let expected: Vec<u8> = EXPECTED_JSON.into();

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
        };

        let view_model_delta = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        println!(
            "got {}\n expected {}",
            String::from_utf8(
                JsonSerializer::default()
                    .serialize_view_model_delta(&view_model_delta)
                    .unwrap()
            )
            .unwrap(),
            EXPECTED_JSON
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
        };

        let expected = hashmap! { 12 => ObjectDelta::Updated(object_description_delta) };

        let source: Vec<u8> = EXPECTED_JSON.into();

        println!(
            "got {}\n expected {}",
            String::from_utf8(
                JsonSerializer::default()
                    .serialize_view_model_delta(&expected)
                    .unwrap()
            )
            .unwrap(),
            EXPECTED_JSON
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
