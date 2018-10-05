use crate::controller::Presenter;
use crate::serialize::ViewModelSerializer;
use crate::transmitter::ViewModelTransmitter;
use crate::view_model::{self, ViewModel};
use myelin_environment::object as business_object;
use std::error::Error;
use std::fmt;

pub(crate) trait View: fmt::Debug {
    fn draw_objects(&self, view_model: &ViewModel);
    fn flush(&self);
}

#[derive(Debug)]
pub(crate) struct CanvasPresenter {
    serializer: Box<dyn ViewModelSerializer>,
    transmitter: Box<dyn ViewModelTransmitter>,
}

impl Presenter for CanvasPresenter {
    fn present_objects(
        &self,
        objects: &[business_object::ObjectDescription],
    ) -> Result<(), Box<dyn Error>> {
        let view_model = ViewModel {
            objects: objects
                .iter()
                .map(|object| to_global_object(object))
                .collect(),
        };

        self.transmitter
            .send_view_model(self.serializer.serialize_view_model(&view_model)?)?;

        Ok(())
    }
}

fn to_global_object(object: &business_object::ObjectDescription) -> view_model::Object {
    view_model::Object {
        shape: view_model::Polygon {
            vertices: object
                .shape
                .vertices
                .iter()
                .map(|vertex| to_global_rotated_vertex(vertex, object))
                .collect(),
        },
        kind: map_kind(object.kind),
    }
}

fn to_global_rotated_vertex(
    vertex: &business_object::Vertex,
    object: &business_object::ObjectDescription,
) -> view_model::Vertex {
    // algorithm source: https://stackoverflow.com/questions/786472/rotate-a-point-by-another-point-in-2d/786508#786508
    let center_x = f64::from(object.position.location.x);
    let center_y = f64::from(object.position.location.y);
    let rotation = object.position.rotation.0;
    let global_x = center_x + f64::from(vertex.x);
    let global_y = center_y + f64::from(vertex.y);
    let rotated_global_x =
        rotation.cos() * (global_x - center_x) - rotation.sin() * (global_y - center_y) + center_x;
    let rotated_global_y =
        rotation.sin() * (global_x - center_x) + rotation.cos() * (global_y - center_y) + center_y;

    view_model::Vertex {
        x: rotated_global_x.round() as u32,
        y: rotated_global_y.round() as u32,
    }
}

fn map_kind(kind: business_object::Kind) -> view_model::Kind {
    match kind {
        business_object::Kind::Organism => view_model::Kind::Organism,
        business_object::Kind::Plant => view_model::Kind::Plant,
        business_object::Kind::Water => view_model::Kind::Water,
        business_object::Kind::Terrain => view_model::Kind::Terrain,
    }
}

impl CanvasPresenter {
    pub(crate) fn new(
        serializer: Box<dyn ViewModelSerializer>,
        transmitter: Box<dyn ViewModelTransmitter>,
    ) -> Self {
        Self {
            transmitter,
            serializer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_model::{self, ViewModel};
    use myelin_environment::object::{Kind, Mobility, ObjectDescription, Radians};
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::cell::RefCell;
    use std::error::Error;
    use std::f64::consts::PI;
    use std::thread;

    #[derive(Debug)]
    struct TransmitterMock {
        expected_data: Vec<u8>,
        send_view_model_was_called: RefCell<bool>,
    }

    impl TransmitterMock {
        fn new(expected_data: Vec<u8>) -> Self {
            Self {
                expected_data,
                send_view_model_was_called: RefCell::new(false),
            }
        }
    }

    impl ViewModelTransmitter for TransmitterMock {
        fn send_view_model(&self, view_model: Vec<u8>) -> Result<(), Box<dyn Error>> {
            assert_eq!(self.expected_data, view_model);

            *self.send_view_model_was_called.borrow_mut() = true;

            Ok(())
        }
    }

    impl Drop for TransmitterMock {
        fn drop(&mut self) {
            if !thread::panicking() {
                assert!(*self.send_view_model_was_called.borrow());
            }
        }
    }

    #[derive(Debug)]
    struct SerializerMock {
        expected_view_model: ViewModel,
        return_value: Vec<u8>,
        serialize_view_model_was_called: RefCell<bool>,
    }

    impl SerializerMock {
        fn new(expected_view_model: ViewModel, return_value: Vec<u8>) -> Self {
            Self {
                expected_view_model,
                return_value,
                serialize_view_model_was_called: RefCell::new(false),
            }
        }
    }

    impl ViewModelSerializer for SerializerMock {
        fn serialize_view_model(&self, view_model: &ViewModel) -> Result<Vec<u8>, Box<dyn Error>> {
            assert_eq!(&self.expected_view_model, view_model);

            *self.serialize_view_model_was_called.borrow_mut() = true;

            Ok(self.return_value.clone())
        }
    }

    impl Drop for SerializerMock {
        fn drop(&mut self) {
            if !thread::panicking() {
                assert!(*self.serialize_view_model_was_called.borrow());
            }
        }
    }

    fn object_description(orientation: Radians) -> ObjectDescription {
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-10, -10)
                    .vertex(10, -10)
                    .vertex(10, 10)
                    .vertex(-10, 10)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Immovable)
            .location(30, 40)
            .rotation(orientation)
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    #[test]
    fn maps_to_empty_view_model() {
        let objects = Vec::new();
        let expected_view_model = ViewModel {
            objects: Vec::new(),
        };
        let expected_data = vec![1, 2, 3];
        let serializer_mock = SerializerMock::new(expected_view_model, expected_data.clone());
        let transmitter_mock = TransmitterMock::new(expected_data);
        let presenter = CanvasPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        presenter.present_objects(&objects).unwrap();
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let object_description = [object_description(Radians::default())];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex { x: 20, y: 30 },
                        view_model::Vertex { x: 40, y: 30 },
                        view_model::Vertex { x: 40, y: 50 },
                        view_model::Vertex { x: 20, y: 50 },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let expected_data = vec![1, 2, 3];
        let serializer_mock = SerializerMock::new(expected_view_model, expected_data.clone());
        let transmitter_mock = TransmitterMock::new(expected_data);
        let presenter = CanvasPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        presenter.present_objects(&object_description).unwrap();
    }

    #[test]
    fn converts_to_global_object_with_pi_orientation() {
        let object_description = [object_description(Radians(PI))];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex { x: 40, y: 50 },
                        view_model::Vertex { x: 20, y: 50 },
                        view_model::Vertex { x: 20, y: 30 },
                        view_model::Vertex { x: 40, y: 30 },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let expected_data = vec![1, 2, 3];
        let serializer_mock = SerializerMock::new(expected_view_model, expected_data.clone());
        let transmitter_mock = TransmitterMock::new(expected_data);
        let presenter = CanvasPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        presenter.present_objects(&object_description).unwrap();
    }

    #[test]
    fn converts_to_global_object_with_arbitrary_orientation() {
        let object_description = [object_description(Radians(3.0))];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex {
                            x: 40 + 1,
                            y: 50 - 2,
                        },
                        view_model::Vertex {
                            x: 20 + 2,
                            y: 50 + 1,
                        },
                        view_model::Vertex {
                            x: 20 - 1,
                            y: 30 + 2,
                        },
                        view_model::Vertex {
                            x: 40 - 2,
                            y: 30 - 1,
                        },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let expected_data = vec![1, 2, 3];
        let serializer_mock = SerializerMock::new(expected_view_model, expected_data.clone());
        let transmitter_mock = TransmitterMock::new(expected_data);
        let presenter = CanvasPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        presenter.present_objects(&object_description).unwrap();
    }
}
