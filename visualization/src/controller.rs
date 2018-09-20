use myelin_environment::object::ObjectDescription;
use myelin_environment::simulation::Simulation;
use myelin_worldgen::WorldGenerator;

pub(crate) trait Controller {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[ObjectDescription]);
}

pub(crate) struct ControllerImpl {
    presenter: Box<dyn Presenter>,
    simulation: Box<dyn Simulation>,
}

impl Controller for ControllerImpl {
    fn step(&mut self) {
        self.simulation.step();
        let objects = self.simulation.objects();
        self.presenter.present_objects(&objects);
    }
}

impl ControllerImpl {
    pub(crate) fn new(presenter: Box<dyn Presenter>, world_generator: &dyn WorldGenerator) -> Self {
        Self {
            presenter,
            simulation: world_generator.generate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::simulation::NewObject;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct SimulationMock {
        step_was_called: bool,
        returned_objects: Vec<ObjectDescription>,
        objects_was_called: RefCell<bool>,
    }
    impl SimulationMock {
        fn new(returned_objects: Vec<ObjectDescription>) -> Self {
            Self {
                step_was_called: false,
                objects_was_called: RefCell::new(false),
                returned_objects,
            }
        }
    }
    impl Simulation for SimulationMock {
        fn step(&mut self) {
            self.step_was_called = true;
        }
        fn add_object(&mut self, _: NewObject) {
            panic!("add_object() was called unexpectedly")
        }
        fn set_simulated_timestep(&mut self, _: f64) {
            panic!("set_simulated_timestep() called unexpectedly");
        }
        fn objects(&self) -> Vec<ObjectDescription> {
            *self.objects_was_called.borrow_mut() = true;
            self.returned_objects.clone()
        }
    }

    impl Drop for SimulationMock {
        fn drop(&mut self) {
            assert!(*self.objects_was_called.borrow());
            assert!(self.step_was_called);
        }
    }

    #[derive(Debug)]
    struct PresenterMock {
        expected_objects: Vec<ObjectDescription>,
        present_objects_was_called: RefCell<bool>,
    }
    impl PresenterMock {
        fn new(expected_objects: Vec<ObjectDescription>) -> Self {
            Self {
                present_objects_was_called: RefCell::new(false),
                expected_objects,
            }
        }
    }
    impl Presenter for PresenterMock {
        fn present_objects(&self, objects: &[ObjectDescription]) {
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

    struct WorldGeneratorMock {
        simulation_factory: Box<dyn Fn(Vec<ObjectDescription>) -> Box<dyn Simulation>>,
        generate_was_called: RefCell<bool>,
        objects_to_return: Vec<ObjectDescription>,
    }
    impl WorldGeneratorMock {
        fn new(
            simulation_factory: Box<dyn Fn(Vec<ObjectDescription>) -> Box<dyn Simulation>>,
            objects_to_return: Vec<ObjectDescription>,
        ) -> Self {
            Self {
                generate_was_called: RefCell::new(false),
                simulation_factory,
                objects_to_return,
            }
        }
    }
    impl WorldGenerator for WorldGeneratorMock {
        fn generate(&self) -> Box<dyn Simulation> {
            *self.generate_was_called.borrow_mut() = true;
            (self.simulation_factory)(self.objects_to_return.clone())
        }
    }

    impl Drop for WorldGeneratorMock {
        fn drop(&mut self) {
            assert!(*self.generate_was_called.borrow());
        }
    }

    fn mock_controller(expected_objects: Vec<ObjectDescription>) -> ControllerImpl {
        let simulation_factory = Box::new(|objects_present| -> Box<dyn Simulation> {
            Box::new(SimulationMock::new(objects_present))
        });
        let world_generator = WorldGeneratorMock::new(simulation_factory, expected_objects.clone());
        let presenter: PresenterMock = PresenterMock::new(expected_objects);
        ControllerImpl::new(Box::new(presenter), &world_generator)
    }

    #[test]
    fn propagates_empty_step() {
        let expected_objects = vec![];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }

    #[test]
    fn propagates_step() {
        let expected_objects = vec![ObjectDescription {
            shape: Polygon {
                vertices: vec![
                    Vertex { x: -5, y: -5 },
                    Vertex { x: 5, y: -5 },
                    Vertex { x: 5, y: 5 },
                    Vertex { x: -5, y: 5 },
                ],
            },
            position: Position {
                location: Location { x: 20, y: 40 },
                rotation: Radians(6.0),
            },
            velocity: Mobility::Movable(Velocity { x: 0, y: -1 }),
            kind: Kind::Organism,
        }];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }
}
