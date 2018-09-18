use myelin_environment::object::{GlobalBody, GlobalObject};
use myelin_environment::simulation::Simulation;
use myelin_worldgen::WorldGenerator;

pub(crate) trait Controller {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[GlobalObject<'_>]);
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
    use std::cell::RefCell;

    #[derive(Debug)]
    struct SimulationMock {
        step_was_called: bool,
        returned_objects: Vec<GlobalObject>,
        objects_was_called: RefCell<bool>,
    }
    impl SimulationMock {
        fn new(returned_objects: Vec<GlobalObject>) -> Self {
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
        fn add_object(&mut self, _: LocalObject) {
            panic!("add_object() was called unexpectedly")
        }
        fn set_simulated_timestep(&mut self, _: f64) {
            panic!("set_simulated_timestep() called unexpectedly");
        }
        fn objects(&self) -> Vec<GlobalObject<'_>> {
            *self.objects_was_called.borrow_mut() = true;
            self.returned_objects
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
        expected_objects: Vec<GlobalObject>,
        present_objects_was_called: RefCell<bool>,
    }
    impl PresenterMock {
        fn new(expected_objects: Vec<GlobalObject>) -> Self {
            Self {
                present_objects_was_called: RefCell::new(false),
                expected_objects,
            }
        }
    }
    impl Presenter for PresenterMock {
        fn present_objects(&self, objects: &[GlobalObject<'_>]) {
            *self.present_objects_was_called.borrow_mut() = true;
            self.expected_objects
                .iter()
                .zip(objects)
                .for_each(|(expected, actual)| {
                    assert_eq!(expected.body, actual.body);
                });
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            assert!(*self.present_objects_was_called.borrow());
        }
    }

    struct WorldGeneratorMock {
        simulation_factory: Box<dyn FnOnce() -> Box<dyn Simulation>>,
        generate_was_called: RefCell<bool>,
    }
    impl WorldGeneratorMock {
        fn new(simulation_factory: Box<dyn FnOnce() -> Box<dyn Simulation>>) -> Self {
            Self {
                generate_was_called: RefCell::new(false),
                simulation_factory,
            }
        }
    }
    impl WorldGenerator for WorldGeneratorMock {
        fn generate(&self) -> Box<dyn Simulation> {
            *self.generate_was_called.borrow_mut() = true;
            (self.simulation_factory)()
        }
    }

    impl Drop for WorldGeneratorMock {
        fn drop(&mut self) {
            assert!(*self.generate_was_called.borrow());
        }
    }

    fn mock_controller(expected_objects: Vec<GlobalObject>) -> ControllerImpl {
        let simulation_factory = Box::new(move || -> Box<dyn Simulation> {
            Box::new(SimulationMock::new(expected_objects))
        });
        let world_generator = WorldGeneratorMock::new(simulation_factory);
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
    fn propagates_step_step() {
        let expected_objects = vec![GlobalObject {
            body: GlobalBody {
                orientation: Radians(6.0),
                shape: GlobalPolygon {
                    vertices: vec![
                        GlobalVertex { x: 2, y: 3 },
                        GlobalVertex { x: 10, y: 3 },
                        GlobalVertex { x: 30, y: 34 },
                    ],
                },
                velocity: Velocity { x: 0, y: -1 },
            },
            behavior: unimplemented!(),
        }];
        let mut controller = mock_controller(expected_objects);
        controller.step();
    }
}
