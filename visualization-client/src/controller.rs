use crate::input_handler::Controller;
use myelin_visualization_core::serialization::ViewModelDeserializer;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::error::Error;
use std::fmt;

pub(crate) trait Presenter: fmt::Debug {
    fn present_delta(&mut self, delta: ViewModelDelta) -> Result<(), Box<dyn Error>>;
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

        self.presenter.present_delta(view_model_delta)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_geometry::*;
    use myelin_visualization_core::view_model_delta::{ObjectDelta, ObjectDescriptionDelta};
    use std::cell::RefCell;
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
        fn present_delta(&mut self, delta: ViewModelDelta) -> Result<(), Box<dyn Error>> {
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
            kind: None,
            sensor: None,
        }
    }

    #[test]
    fn deserializes_and_calls_presenter() {
        let data = vec![100, 124, 135, 253, 234, 122];
        let view_model_delta = hashmap! {
            123 => ObjectDelta::Updated(object_description_delta())
        };

        let view_model_deserializer =
            ViewModelDeserializerMock::new(data.clone(), view_model_delta.clone());
        let presenter = PresenterMock::new(view_model_delta.clone());
        let mut controller = ControllerImpl::new(box presenter, box view_model_deserializer);

        controller.on_message(&data).unwrap();
    }
}
