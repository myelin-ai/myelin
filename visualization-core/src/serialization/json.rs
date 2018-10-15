use crate::serialization::{ViewModelDeserializer, ViewModelSerializer};
use crate::view_model_delta::ViewModelDelta;
use std::error::Error;

#[derive(Debug)]
pub struct JsonSerializer;

#[derive(Debug)]
pub struct JsonDeserializer;

impl JsonSerializer {
    pub fn new() -> Self {
        JsonSerializer
    }
}

impl JsonDeserializer {
    pub fn new() -> Self {
        JsonDeserializer
    }
}

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
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>> {
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
        let expected: Vec<u8> = r#"{"objects":[{"shape":{"vertices":[{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"kind":"Organism"}]}"#.into();

        let view_model_delta = ViewModelDelta {
            objects: vec![ObjectDescriptionDelta {
                id: 12,
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
                position: Some(Position {
                    location: Location { x: 3, y: 4 },
                    rotation: Radians::new(1.0).unwrap(),
                }),
                sensor: Some(Some(Sensor {
                    shape: PolygonBuilder::new()
                        .vertex(-10, -12)
                        .vertex(10, 6)
                        .vertex(16, 0)
                        .build()
                        .unwrap(),
                    position: Position {
                        location: Location { x: 2, y: 3 },
                        rotation: Radians::new(-1.0).unwrap(),
                    },
                })),
            }],
            deleted_objects: Vec::new(),
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_works_with_empty_view_model() {
        let expected: Vec<u8> = r#"{"objects":[]}"#.into();

        let view_model_delta = ViewModelDelta {
            objects: Vec::new(),
            deleted_objects: Vec::new(),
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn deserializes_full_viewmodel() {
        let expected = ViewModelDelta {
            objects: vec![ObjectDescriptionDelta {
                id: 12,
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
                position: Some(Position {
                    location: Location { x: 3, y: 4 },
                    rotation: Radians::new(1.0).unwrap(),
                }),
                sensor: Some(Some(Sensor {
                    shape: PolygonBuilder::new()
                        .vertex(-10, -12)
                        .vertex(10, 6)
                        .vertex(16, 0)
                        .build()
                        .unwrap(),
                    position: Position {
                        location: Location { x: 2, y: 3 },
                        rotation: Radians::new(-1.0).unwrap(),
                    },
                })),
            }],
            deleted_objects: Vec::new(),
        };

        let source: Vec<u8> = r#"{"objects":[{"shape":{"vertices":[{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"kind":"Organism"}]}"#.into();

        let deserializer = JsonDeserializer::new();
        let deserialized = deserializer.deserialize_view_model(source).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn deserialize_works_with_empty_view_model() {
        let expected = ViewModelDelta {
            objects: Vec::new(),
            deleted_objects: Vec::new(),
        };

        let source: Vec<u8> = r#"{"objects":[]}"#.into();

        let deserializer = JsonDeserializer::new();
        let deserialized = deserializer.deserialize_view_model(source).unwrap();

        assert_eq!(expected, deserialized);
    }
}
