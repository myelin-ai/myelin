use crate::input_handler::Controller;
use crate::presenter;
use myelin_environment::object::ObjectDescription;
use myelin_object_data::{
    AssociatedObjectData, AssociatedObjectDataDeserializer, AssociatedObjectDataSerializer,
};
use myelin_visualization_core::serialization::ViewModelDeserializer;
use myelin_visualization_core::view_model_delta::{
    ObjectDelta, ObjectDescriptionDelta, ViewModelDelta,
};
use std::error::Error;
use std::fmt;

pub(crate) trait Presenter: fmt::Debug {
    fn present_delta(&mut self, delta: presenter::ViewModelDelta) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub(crate) struct ControllerImpl {
    presenter: Box<dyn Presenter>,
    view_model_deserializer: Box<dyn ViewModelDeserializer>,
    associated_object_data_serializer: Box<dyn AssociatedObjectDataSerializer>,
    associated_object_data_deserializer: Box<dyn AssociatedObjectDataDeserializer>,
}

impl Controller for ControllerImpl {
    fn on_message(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let view_model_delta = self
            .view_model_deserializer
            .deserialize_view_model_delta(message)?;

        self.presenter.present_delta(translate_delta(
            view_model_delta,
            self.associated_object_data_deserializer.as_ref(),
        ))?;

        Ok(())
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        presenter: Box<dyn Presenter>,
        view_model_deserializer: Box<dyn ViewModelDeserializer>,
        associated_object_data_serializer: Box<dyn AssociatedObjectDataSerializer>,
        associated_object_data_deserializer: Box<dyn AssociatedObjectDataDeserializer>,
    ) -> Self {
        Self {
            presenter,
            view_model_deserializer,
            associated_object_data_serializer,
            associated_object_data_deserializer,
        }
    }
}

fn translate_delta(
    delta: ViewModelDelta,
    associated_object_data_deserializer: &dyn AssociatedObjectDataDeserializer,
) -> presenter::ViewModelDelta {
    delta
        .into_iter()
        .map(|(id, object_delta)| {
            let object_delta = match object_delta {
                ObjectDelta::Deleted => presenter::ObjectDelta::Deleted,
                ObjectDelta::Created(object_description) => {
                    presenter::ObjectDelta::Created(translate_object_description(
                        object_description,
                        associated_object_data_deserializer,
                    ))
                }
                ObjectDelta::Updated(object_description_delta) => {
                    presenter::ObjectDelta::Updated(translate_object_description_delta(
                        object_description_delta,
                        associated_object_data_deserializer,
                    ))
                }
            };

            (id, object_delta)
        })
        .collect()
}

fn translate_object_description(
    object_description: ObjectDescription,
    associated_object_data_deserializer: &dyn AssociatedObjectDataDeserializer,
) -> presenter::ObjectDescription {
    let associated_object_data = associated_object_data_deserializer
        .deserialize(&object_description.associated_data)
        .expect("Unable to deserialize associated object data");

    let AssociatedObjectData { name, kind } = associated_object_data;

    let ObjectDescription {
        shape,
        location,
        rotation,
        mobility,
        passable,
        ..
    } = object_description;

    presenter::ObjectDescription {
        name,
        kind,
        shape,
        location,
        rotation,
        mobility,
        passable,
    }
}

fn translate_object_description_delta(
    object_description_delta: ObjectDescriptionDelta,
    associated_object_data_deserializer: &dyn AssociatedObjectDataDeserializer,
) -> presenter::ObjectDescriptionDelta {
    let associated_object_data: Option<AssociatedObjectData> = object_description_delta
        .associated_data
        .map(|associated_data| {
            associated_object_data_deserializer
                .deserialize(&associated_data)
                .expect("Unable to deserialize associated data")
        });

    let (name, kind) = if let Some(associated_object_data) = associated_object_data {
        (
            Some(associated_object_data.name),
            Some(associated_object_data.kind),
        )
    } else {
        (None, None)
    };

    let ObjectDescriptionDelta {
        shape,
        location,
        rotation,
        mobility,
        ..
    } = object_description_delta;

    presenter::ObjectDescriptionDelta {
        name,
        kind,
        shape,
        location,
        rotation,
        mobility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::{partial_eq_owned};
    use myelin_geometry::*;
    use myelin_object_data::{AssociatedObjectDataSerializerMock, Kind};
    use myelin_visualization_core::view_model_delta::{ObjectDelta, ObjectDescriptionDelta};
    use std::cell::RefCell;
    use std::error::Error;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    #[derive(Debug)]
    struct PresenterMock {
        expected_view_model_delta: presenter::ViewModelDelta,
        present_delta_was_called: RefCell<bool>,
    }

    impl PresenterMock {
        fn new(expected_view_model_delta: presenter::ViewModelDelta) -> Self {
            Self {
                present_delta_was_called: RefCell::new(false),
                expected_view_model_delta,
            }
        }
    }

    impl Presenter for PresenterMock {
        fn present_delta(
            &mut self,
            delta: presenter::ViewModelDelta,
        ) -> Result<(), Box<dyn Error>> {
            *self.present_delta_was_called.borrow_mut() = true;
            assert_eq!(self.expected_view_model_delta, delta);
            Ok(())
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            if !panicking() {
                assert!(
                    *self.present_delta_was_called.borrow(),
                    "present_delta() was never called, but was expected"
                );
            }
        }
    }

    #[derive(Debug)]
    struct ViewModelDeserializerMock {
        expected_data: Vec<u8>,
        view_model_delta: ViewModelDelta,
        deserialize_view_model_was_called: RefCell<bool>,
    }

    impl ViewModelDeserializerMock {
        fn new(expected_data: Vec<u8>, view_model_delta: ViewModelDelta) -> Self {
            Self {
                expected_data,
                view_model_delta,
                deserialize_view_model_was_called: RefCell::new(false),
            }
        }
    }

    impl ViewModelDeserializer for ViewModelDeserializerMock {
        fn deserialize_view_model_delta(
            &self,
            buf: &[u8],
        ) -> Result<ViewModelDelta, Box<dyn Error>> {
            *self.deserialize_view_model_was_called.borrow_mut() = true;
            assert_eq!(self.expected_data, buf);
            Ok(self.view_model_delta.clone())
        }
    }

    impl Drop for ViewModelDeserializerMock {
        fn drop(&mut self) {
            if !panicking() {
                assert!(
                    *self.deserialize_view_model_was_called.borrow(),
                    "deserialize_view_model() was never called, but was expected"
                );
            }
        }
    }

    fn object_description_delta() -> ObjectDescriptionDelta {
        let mut associated_object_data_serializer = AssociatedObjectDataSerializerMock::new();

        associated_object_data_serializer
            .expect_serialize(partial_eq_owned(AssociatedObjectData {
                name: Some(String::from("Cat")),
                kind: Kind::Organism,
            }))
            .returns(String::from("A very pretty looking cat").into_bytes());

        ObjectDescriptionDelta {
            shape: Some(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(5.0, -5.0)
                    .vertex(5.0, 5.0)
                    .vertex(-5.0, 5.0)
                    .build()
                    .expect("Created invalid vertex"),
            ),
            location: Some(Point { x: 20.0, y: 40.0 }),
            rotation: Some(Radians::try_new(6.0).unwrap()),
            mobility: None,
            associated_data: Some(associated_object_data_serializer.serialize(
                &AssociatedObjectData {
                    name: Some(String::from("Cat")),
                    kind: Kind::Organism,
                },
            )),
        }
    }

    fn presenter_object_description_delta() -> presenter::ObjectDescriptionDelta {
        presenter::ObjectDescriptionDelta {
            name: Some(Some(String::from("Cat"))),
            kind: Some(Kind::Organism),
            shape: Some(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(5.0, -5.0)
                    .vertex(5.0, 5.0)
                    .vertex(-5.0, 5.0)
                    .build()
                    .expect("Created invalid vertex"),
            ),
            location: Some(Point { x: 20.0, y: 40.0 }),
            rotation: Some(Radians::try_new(6.0).unwrap()),
            ..Default::default()
        }
    }

    #[derive(Debug, Default)]
    struct AssociatedObjectDataDeserializerMock {
        expect_deserialize: Option<(Vec<u8>, AssociatedObjectData)>,
        expect_deserialize_called: AtomicBool,
    }

    impl Drop for AssociatedObjectDataDeserializerMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.expect_deserialize.is_some() && !self.expect_deserialize_called.load(Ordering::SeqCst) {
                    panic!("Expected method was not called")
                }
            }
        }
    }

    impl AssociatedObjectDataDeserializerMock {
        fn new(parameter: Vec<u8>, return_value: AssociatedObjectData) -> Self {
            Self {
                expect_deserialize: Some((parameter, return_value)),
                expect_deserialize_called: Default::default(),
            }
        }
    }

    impl AssociatedObjectDataDeserializer for AssociatedObjectDataDeserializerMock {
        fn deserialize(&self, data: &[u8]) -> Result<AssociatedObjectData, Box<dyn Error>> {
            if let Some((parameter, return_value)) = &self.expect_deserialize {
                if parameter.as_slice() != data {
                    panic!("Was called with {:?}, but expected {:?}", data, parameter)
                } else {
                    self.expect_deserialize_called.store(true, Ordering::SeqCst);
                    Ok(return_value.clone())
                }
            } else {
                panic!("Call was not expected")
            }
        }
    }

    #[test]
    fn deserializes_and_calls_presenter() {
        let data = vec![100, 124, 135, 253, 234, 122];
        let presenter_view_model_delta = hashmap! {
            123 => presenter::ObjectDelta::Updated(presenter_object_description_delta())
        };

        let view_model_delta = hashmap! {
            123 => ObjectDelta::Updated(object_description_delta())
        };

        let associated_ojbect_data_bytes = String::from("A very pretty looking cat").into_bytes();
        let associated_object_data = AssociatedObjectData {
            name: Some(String::from("Cat")),
            kind: Kind::Organism,
        };

        let associated_object_data_serializer = AssociatedObjectDataSerializerMock::new();
        let associated_object_data_deserializer = AssociatedObjectDataDeserializerMock::new(
            associated_ojbect_data_bytes, associated_object_data
        );

        let view_model_deserializer =
            ViewModelDeserializerMock::new(data.clone(), view_model_delta.clone());
        let presenter = PresenterMock::new(presenter_view_model_delta.clone());
        let mut controller = ControllerImpl::new(
            box presenter,
            box view_model_deserializer,
            box associated_object_data_serializer,
            box associated_object_data_deserializer,
        );

        controller.on_message(&data).unwrap();
    }

}
