use myelin_environment::object::GlobalObject;
use myelin_environment::world::World;
use myelin_worldgen::WorldGenerator;

pub(crate) trait Simulation {
    fn step(&mut self);
}
pub(crate) trait Presenter {
    fn present_objects(&self, objects: &[GlobalObject]);
}

pub(crate) struct SimulationImpl {
    presenter: Box<dyn Presenter>,
    world: Box<dyn World>,
}

impl Simulation for SimulationImpl {
    fn step(&mut self) {
        self.world.step();
        let objects = self.world.objects();
        self.presenter.present_objects(&objects);
    }
}

impl SimulationImpl {
    pub(crate) fn new(
        presenter: Box<dyn Presenter>,
        world_generator: Box<dyn WorldGenerator>,
    ) -> Self {
        Self {
            presenter,
            world: world_generator.generate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct WorldMock {
        step_was_called: bool,
        returned_objects: Vec<GlobalObject>,
        objects_was_called: RefCell<bool>,
    }
    impl WorldMock {
        fn new(returned_objects: Vec<GlobalObject>) -> Self {
            Self {
                step_was_called: false,
                objects_was_called: RefCell::new(false),
                returned_objects,
            }
        }
    }
    impl World for WorldMock {
        fn step(&mut self) {
            self.step_was_called = true;
        }
        fn add_object(&mut self, _: LocalObject) {
            panic!("add_object() was called unexpectedly")
        }
        fn objects(&self) -> Vec<GlobalObject> {
            *self.objects_was_called.borrow_mut() = true;
            self.returned_objects.clone()
        }
    }

    impl Drop for WorldMock {
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
        fn present_objects(&self, objects: &[GlobalObject]) {
            *self.present_objects_was_called.borrow_mut() = true;
            assert_eq!(self.expected_objects, objects.to_vec());
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            assert!(*self.present_objects_was_called.borrow());
        }
    }

    #[derive(Debug)]
    struct WorldGeneratorMock {
        expected_objects: Vec<GlobalObject>,
        generate_was_called: RefCell<bool>,
    }
    impl WorldGeneratorMock {
        fn new(expected_objects: Vec<GlobalObject>) -> Self {
            Self {
                generate_was_called: RefCell::new(false),
                expected_objects,
            }
        }
    }
    impl WorldGenerator for WorldGeneratorMock {
        fn generate(&self) -> Box<dyn World> {
            *self.generate_was_called.borrow_mut() = true;
            Box::new(WorldMock::new(self.expected_objects.clone()))
        }
    }

    impl Drop for WorldGeneratorMock {
        fn drop(&mut self) {
            assert!(*self.generate_was_called.borrow());
        }
    }

    #[test]
    fn propagates_empty_step() {
        let expected_objects = vec![];
        let world_generator = WorldGeneratorMock::new(expected_objects.clone());
        let presenter = PresenterMock::new(expected_objects);
        let mut simulation = SimulationImpl::new(Box::new(presenter), Box::new(world_generator));
        simulation.step();
    }

    #[test]
    fn propagates_step_step() {
        let expected_objects = vec![GlobalObject {
            orientation: Radians(6.0),
            shape: GlobalPolygon {
                vertices: vec![
                    GlobalVertex { x: 2, y: 3 },
                    GlobalVertex { x: 10, y: 3 },
                    GlobalVertex { x: 30, y: 34 },
                ],
            },
            velocity: Velocity { x: 0, y: -1 },
            kind: Kind::Plant,
        }];
        let world_generator = WorldGeneratorMock::new(expected_objects.clone());
        let presenter = PresenterMock::new(expected_objects);
        let mut simulation = SimulationImpl::new(Box::new(presenter), Box::new(world_generator));
        simulation.step();
    }
}
