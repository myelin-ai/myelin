use myelin_environment::object::{GlobalBody, GlobalObject};
use myelin_environment::simulation::Simulation;
use myelin_worldgen::WorldGenerator;

pub(crate) trait Controller {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[GlobalObject<'_>]);
}

pub(crate) struct ControllerImpl<'a> {
    presenter: Box<dyn Presenter + 'a>,
    simulation: Box<dyn Simulation + 'a>,
}

impl<'a> Controller for ControllerImpl<'a> {
    fn step(&mut self) {
        self.simulation.step();
        let objects = self.simulation.objects();
        self.presenter.present_objects(&objects);
    }
}

impl<'a> ControllerImpl<'a> {
    pub(crate) fn new(
        presenter: Box<dyn Presenter + 'a>,
        world_generator: &dyn WorldGenerator<'a>,
    ) -> Self {
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
    struct SimulationMock<'a> {
        step_was_called: bool,
        returned_objects: Vec<GlobalObject<'a>>,
        objects_was_called: RefCell<bool>,
    }
    impl<'a> SimulationMock<'a> {
        fn new(returned_objects: Vec<GlobalObject<'a>>) -> Self {
            Self {
                step_was_called: false,
                objects_was_called: RefCell::new(false),
                returned_objects,
            }
        }
    }
    impl<'a> Simulation for SimulationMock<'a> {
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

    impl<'a> Drop for SimulationMock<'a> {
        fn drop(&mut self) {
            assert!(*self.objects_was_called.borrow());
            assert!(self.step_was_called);
        }
    }

    #[derive(Debug)]
    struct PresenterMock<'a> {
        expected_objects: Vec<GlobalObject<'a>>,
        present_objects_was_called: RefCell<bool>,
    }
    impl<'a> PresenterMock<'a> {
        fn new(expected_objects: Vec<GlobalObject<'a>>) -> Self {
            Self {
                present_objects_was_called: RefCell::new(false),
                expected_objects,
            }
        }
    }
    impl<'a> Presenter for PresenterMock<'a> {
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

    impl<'a> Drop for PresenterMock<'a> {
        fn drop(&mut self) {
            assert!(*self.present_objects_was_called.borrow());
        }
    }

    struct WorldGeneratorMock<'a> {
        simulation_factory: Box<dyn FnOnce() -> Box<dyn Simulation + 'a> + 'a>,
        generate_was_called: RefCell<bool>,
    }
    impl<'a> WorldGeneratorMock<'a> {
        fn new(simulation_factory: Box<dyn FnOnce() -> Box<dyn Simulation + 'a> + 'a>) -> Self {
            Self {
                generate_was_called: RefCell::new(false),
                simulation_factory,
            }
        }
    }
    impl<'a> WorldGenerator<'a> for WorldGeneratorMock<'a> {
        fn generate(&self) -> Box<dyn Simulation + 'a> {
            *self.generate_was_called.borrow_mut() = true;
            (self.simulation_factory)()
        }
    }

    impl<'a> Drop for WorldGeneratorMock<'a> {
        fn drop(&mut self) {
            assert!(*self.generate_was_called.borrow());
        }
    }

    fn mock_controller<'a>(expected_objects: Vec<GlobalObject<'a>>) -> ControllerImpl<'a> {
        let simulation_factory = Box::new(move || -> Box<dyn Simulation> {
            Box::new(SimulationMock::new(expected_objects))
        });
        let world_generator = WorldGeneratorMock::new(simulation_factory);
        let presenter: PresenterMock<'a> = PresenterMock::new(expected_objects);
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
