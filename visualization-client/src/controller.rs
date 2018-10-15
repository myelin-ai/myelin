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
    fn on_message(&mut self, _message: &[u8]) {
        unimplemented!()
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
    use myelin_environment::object_builder::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct PresenterMock {
        expected_objects: Vec<ViewModelDelta>,
        present_objects_was_called: RefCell<bool>,
    }

    impl PresenterMock {
        fn new(expected_objects: Vec<ViewModelDelta>) -> Self {
            Self {
                present_objects_was_called: RefCell::new(false),
                expected_objects,
            }
        }
    }

    impl Presenter for PresenterMock {
        fn present_objects(&mut self, objects: &[ViewModelDelta]) {
            *self.present_objects_was_called.borrow_mut() = true;
            self.expected_objects
                .iter()
                .zip(objects)
                .for_each(|(expected, actual)| {
                    assert_eq!(expected, actual);
                });
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            assert!(*self.present_objects_was_called.borrow());
        }
    }

    fn mock_controller(expected_objects: Vec<ObjectDescription>) -> ControllerImpl {
        unimplemented!();
    }

    #[test]
    fn propagates_empty_step() {
        let expected_objects = vec![];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }

    #[test]
    fn propagates_step() {
        let expected_objects = vec![
            ObjectBuilder::new()
                .shape(
                    PolygonBuilder::new()
                        .vertex(-5, -5)
                        .vertex(5, -5)
                        .vertex(5, 5)
                        .vertex(-5, 5)
                        .build()
                        .expect("Created invalid vertex"),
                )
                .location(20, 40)
                .rotation(Radians::new(6.0).unwrap())
                .mobility(Mobility::Movable(Velocity { x: 0, y: -1 }))
                .kind(Kind::Organism)
                .build()
                .expect("Failed to create object"),
        ];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }
}
