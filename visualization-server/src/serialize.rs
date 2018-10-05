use myelin_visualization_core::view_model::ViewModel;
use serde_json as json;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

pub(crate) trait ViewModelSerializer: Debug {
    fn serialize_view_model(&self, view_model: &ViewModel) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub(crate) trait ViewModelDeserializer: Debug {
    fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModel, Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct JsonSerializer(PhantomData<()>);

impl JsonSerializer {
    pub(crate) fn new() -> Self {
        JsonSerializer(PhantomData)
    }
}

impl ViewModelSerializer for JsonSerializer {
    fn serialize_view_model(&self, view_model: &ViewModel) -> Result<Vec<u8>, Box<dyn Error>> {
        let serialized = json::to_string(view_model)?;

        Ok(serialized.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use myelin_visualization_core::view_model::*;

    #[test]
    fn serialize_works() {
        let expected: Vec<u8> = r#"{"objects":[{"shape":{"vertices":[{"x":1,"y":1},{"x":2,"y":3},{"x":5,"y":6}]},"kind":"Organism"}]}"#.into();

        let view_model = ViewModel {
            objects: vec![Object {
                kind: Kind::Organism,
                shape: Polygon {
                    vertices: vec![
                        Vertex { x: 1, y: 1 },
                        Vertex { x: 2, y: 3 },
                        Vertex { x: 5, y: 6 },
                    ],
                },
            }],
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer.serialize_view_model(&view_model).unwrap();

        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_works_with_empty_view_model() {
        let expected: Vec<u8> = r#"{"objects":[]}"#.into();

        let view_model = ViewModel {
            objects: Vec::new(),
        };

        let serializer = JsonSerializer::new();
        let serialized = serializer.serialize_view_model(&view_model).unwrap();

        assert_eq!(expected, serialized);
    }
}
