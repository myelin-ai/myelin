use crate::input_handler::Controller;
use crate::presenter;
use myelin_object_data::{AdditionalObjectDescription, ObjectDescription};
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
}

impl Controller for ControllerImpl {
    fn on_message(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let view_model_delta = self
            .view_model_deserializer
            .deserialize_view_model_delta(message)?;

        self.presenter
            .present_delta(translate_delta(view_model_delta))?;

        Ok(())
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        presenter: Box<dyn Presenter>,
        view_model_deserializer: Box<dyn ViewModelDeserializer>,
    ) -> Self {
        Self {
            presenter,
            view_model_deserializer,
        }
    }
}

fn translate_delta(delta: ViewModelDelta) -> presenter::ViewModelDelta {
    delta
        .into_iter()
        .map(|(id, object_delta)| {
            let object_delta = match object_delta {
                ObjectDelta::Deleted => presenter::ObjectDelta::Deleted,
                ObjectDelta::Created(object_description) => presenter::ObjectDelta::Created(
                    translate_object_description(object_description),
                ),
                ObjectDelta::Updated(object_description_delta) => presenter::ObjectDelta::Updated(
                    translate_object_description_delta(object_description_delta),
                ),
            };

            (id, object_delta)
        })
        .collect()
}

fn translate_object_description(
    object_description: ObjectDescription,
) -> presenter::ObjectDescription {
    let AdditionalObjectDescription { name, kind, height } = object_description.associated_data;

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
        height,
        shape,
        location,
        rotation,
        mobility,
        passable,
    }
}

fn translate_object_description_delta(
    object_description_delta: ObjectDescriptionDelta,
) -> presenter::ObjectDescriptionDelta {
    let (name, kind) = object_description_delta
        .associated_data
        .map(|additional_object_description| {
            (
                Some(additional_object_description.name),
                Some(additional_object_description.kind),
            )
        })
        .unwrap_or_default();

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
    use maplit::hashmap;
    use myelin_engine::geometry::*;
    use myelin_object_data::Kind;
    use myelin_visualization_core::view_model_delta::{ObjectDelta, ObjectDescriptionDelta};
    use std::cell::RefCell;
    use std::error::Error;
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
            associated_data: Some(AdditionalObjectDescription {
                name: Some(String::from("Cat")),
                kind: Kind::Organism,
                height: 0.5,
            }),
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
            ..presenter::ObjectDescriptionDelta::default()
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

        let view_model_deserializer =
            ViewModelDeserializerMock::new(data.clone(), view_model_delta.clone());
        let presenter = PresenterMock::new(presenter_view_model_delta.clone());
        let mut controller = ControllerImpl::new(box presenter, box view_model_deserializer);

        controller.on_message(&data).unwrap();
    }

}
