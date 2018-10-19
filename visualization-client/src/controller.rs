use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::fmt;

pub(crate) trait Controller: fmt::Debug {
    fn step(&mut self);
}
pub(crate) trait Presenter: fmt::Debug {
    fn present_objects(&self, objects: &[ViewModelDelta]);
}

#[derive(Debug)]
pub(crate) struct ControllerImpl {
    presenter: Box<dyn Presenter>,
    // To do:
    // Use receiver and deserializer
}

impl Controller for ControllerImpl {
    fn step(&mut self) {
        unimplemented!()
    }
}

impl ControllerImpl {
    pub(crate) fn new(presenter: Box<dyn Presenter>) -> Self {
        Self { presenter }
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
        fn present_objects(&self, objects: &[ViewModelDelta]) {
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

    #[ignore]
    #[test]
    fn propagates_empty_step() {
        let expected_objects = vec![];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }

    #[ignore]
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
