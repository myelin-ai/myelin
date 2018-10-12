use crate::controller::Presenter;
use crate::serialize::ViewModelSerializer;
use crate::snapshot::SnapshotSlice;
use crate::transmitter::ViewModelTransmitter;
use myelin_visualization_core::view_model_delta::ViewModelDelta;

#[derive(Debug)]
pub(crate) struct DeltaPresenter {
    serializer: Box<dyn ViewModelSerializer>,
    transmitter: Box<dyn ViewModelTransmitter>,
}

impl Presenter for DeltaPresenter {
    fn calculate_deltas(
        &self,
        last_objects: &SnapshotSlice,
        current_objects: &SnapshotSlice,
    ) -> ViewModelDelta {
        unimplemented!()
    }
}

impl DeltaPresenter {
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
    use myelin_environment::object::{Kind, Mobility, ObjectDescription, Radians};
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use myelin_visualization_core::view_model::{self, ViewModel};
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
        let presenter = DeltaPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        //presenter.present_objects(&objects).unwrap();
        unimplemented!()
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
        let presenter = DeltaPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        //presenter.present_objects(&object_description).unwrap();
        unimplemented!()
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
        let presenter = DeltaPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        //presenter.present_objects(&object_description).unwrap();
        unimplemented!()
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
        let presenter = DeltaPresenter::new(Box::new(serializer_mock), Box::new(transmitter_mock));
        //presenter.present_objects(&object_description).unwrap();
        unimplemented!()
    }
}
