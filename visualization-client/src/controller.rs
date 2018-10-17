use crate::input_handler::Controller;
use myelin_visualization_core::serialization::ViewModelDeserializer;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::fmt;

pub(crate) trait Presenter: fmt::Debug {
    fn present_delta(&mut self, delta: ViewModelDelta);
}

#[derive(Debug)]
pub(crate) struct ControllerImpl {
    presenter: Box<dyn Presenter>,
    view_model_deserializer: Box<dyn ViewModelDeserializer>,
}

impl Controller for ControllerImpl {
    fn on_message(&mut self, message: &[u8]) {
        let view_model_delta = self
            .view_model_deserializer
            .deserialize_view_model(message)
            .expect("Serialized view model delta was not valid");

        self.presenter.present_delta(view_model_delta);
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

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::PolygonBuilder;
    use myelin_visualization_core::view_model_delta::ObjectDescriptionDelta;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::error::Error;
    use std::thread::panicking;

    #[derive(Debug)]
    struct PresenterMock {
        expected_view_model_delta: ViewModelDelta,
        present_delta_was_called: RefCell<bool>,
    }

    impl PresenterMock {
        fn new(expected_view_model_delta: ViewModelDelta) -> Self {
            Self {
                present_delta_was_called: RefCell::new(false),
                expected_view_model_delta,
            }
        }
    }

    impl Presenter for PresenterMock {
        fn present_delta(&mut self, delta: ViewModelDelta) {
            *self.present_delta_was_called.borrow_mut() = true;
            assert_eq!(self.expected_view_model_delta, delta);
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            if !panicking() {
                assert!(
                    *self.present_delta_was_called.borrow(),
                    "present_delta() was never called"
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
        fn deserialize_view_model(&self, buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>> {
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
                    "deserialize_view_model() was never called"
                );
            }
        }
    }

    fn object_description_delta() -> ObjectDescriptionDelta {
        ObjectDescriptionDelta {
            shape: Some(
                PolygonBuilder::new()
                    .vertex(-5, -5)
                    .vertex(5, -5)
                    .vertex(5, 5)
                    .vertex(-5, 5)
                    .build()
                    .expect("Created invalid vertex"),
            ),
            location: Some(Location { x: 20, y: 40 }),
            rotation: Some(Radians::new(6.0).unwrap()),
            mobility: None,
            kind: None,
            sensor: None,
        }
    }

    #[test]
    fn deserializes_and_calls_presenter() {
        let data = vec![100, 124, 135, 253, 234, 122];
        let view_model_delta = ViewModelDelta {
            updated_objects: hashmap! {
                123 => object_description_delta(),
            },
            ..Default::default()
        };

        let view_model_deserializer =
            ViewModelDeserializerMock::new(data.clone(), view_model_delta.clone());
        let presenter = PresenterMock::new(view_model_delta.clone());
        let mut controller =
            ControllerImpl::new(Box::new(presenter), Box::new(view_model_deserializer));

        controller.on_message(&data);
    }
}
