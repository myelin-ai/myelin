use crate::view_model_delta::ViewModelDelta;
use serde_json as json;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait ViewModelSerializer: Debug {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub trait ViewModelDeserializer: Debug {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct JsonSerializer(PhantomData<()>);

impl JsonSerializer {
    pub fn new() -> Self {
        JsonSerializer(PhantomData)
    }
}

impl ViewModelSerializer for JsonSerializer {
    fn serialize_view_model_delta(
        &self,
        view_model_delta: &ViewModelDelta,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized = json::to_string(view_model_delta)?;

        Ok(serialized.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::view_model_delta::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::PolygonBuilder;
    use std::collections::HashMap;

    #[ignore]
    #[test]
    fn serializes_full_delta() {
        let expected: Vec<u8> = r#"{"objects":[{"shape":{"vertices":[{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"kind":"Organism"}]}"#.into();

        let mut updated_objects = HashMap::new();
        updated_objects.insert(
            12,
            ObjectDescriptionDelta {
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
                        rotation: Radians::new(-1.0).unwrap(),
                    },
                })),
            },
        );

        let view_model_delta = ViewModelDelta {
            created_objects: HashMap::new(),
            updated_objects,
            deleted_objects: Vec::new(),
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }

    #[ignore]
    #[test]
    fn serialize_works_with_empty_view_model() {
        let expected: Vec<u8> = r#"{"objects":[]}"#.into();

        let view_model_delta = ViewModelDelta {
            created_objects: HashMap::new(),
            updated_objects: HashMap::new(),
            deleted_objects: Vec::new(),
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer
            .serialize_view_model_delta(&view_model_delta)
            .unwrap();

        assert_eq!(expected, serialized);
    }
}
