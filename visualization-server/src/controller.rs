use crate::connection::Connection;
use crate::snapshot::{Snapshot, SnapshotSlice};
use myelin_environment::Simulation;
use myelin_worldgen::WorldGenerator;
use std::error::Error;
use std::fmt::{self, Debug};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) trait Controller: Debug {
    fn run(&mut self);
}

pub(crate) trait Presenter: Debug {
    fn present_objects(
        &self,
        last_objects: &SnapshotSlice,
        current_objects: &SnapshotSlice,
    ) -> Result<(), Box<dyn Error>>;
}

pub(crate) trait ConnectionAccepter: Debug + Send {
    fn run(&mut self, sender: Sender<Connection>);
}

pub(crate) type CurrentSnapshotFn = dyn Fn() -> Snapshot + Send;

pub(crate) trait Client: Debug + Send {
    fn run(&mut self, current_snapshot_fn: Box<CurrentSnapshotFn>);
}

pub(crate) type SimulationFactoryFn = dyn Fn() -> Box<dyn Simulation>;
pub(crate) type CurrentSnapshotFnFactory = dyn Fn() -> CurrentSnapshotFn;

pub(crate) trait ClientFactory {
    fn accept_new_connections(
        &self,
        receiver: Receiver<Connection>,
        current_snapshot_fn_factory: Box<CurrentSnapshotFnFactory>,
    );
}

pub(crate) struct ControllerImpl {
    simulation_factory: Box<SimulationFactoryFn>,
    connection_acceptor: Box<dyn ConnectionAccepter>,
    expected_delta: Duration,
    current_snapshot: Arc<RwLock<Snapshot>>,
}

impl Debug for ControllerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControllerImpl")
            .field("connection_acceptor", &self.connection_acceptor)
            .field("expected_delta", &self.expected_delta)
            .finish()
    }
}

impl Controller for ControllerImpl {
    fn run(&mut self) {
        loop {
            let now = Instant::now();

            if let Err(err) = self.step() {
                error!("Error during step: {}", err);
            }

            let delta = now.elapsed();

            if delta < self.expected_delta {
                thread::sleep(self.expected_delta - delta);
            }
        }
    }
}

impl ControllerImpl {
    fn step(&mut self) -> Result<(), Box<dyn Error>> {
        self.simulation.step();
        let objects = self.simulation.objects();
        self.presenter.present_objects(&objects)?;
        Ok(())
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        presenter: Box<dyn Presenter>,
        world_generator: &dyn WorldGenerator,
        expected_delta: Duration,
    ) -> Self {
        Self {
            presenter,
            simulation: world_generator.generate(),
            expected_delta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::*;
    use std::cell::RefCell;
    use std::error::Error;

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
        fn add_object(&mut self, _: ObjectDescription, _: Box<dyn ObjectBehavior>) {
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
        fn present_objects(&self, objects: &[ObjectDescription]) -> Result<(), Box<dyn Error>> {
            *self.present_objects_was_called.borrow_mut() = true;
            self.expected_objects
                .iter()
                .zip(objects)
                .for_each(|(expected, actual)| {
                    assert_eq!(expected, actual);
                });

            Ok(())
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
        ControllerImpl::new(
            Box::new(presenter),
            &world_generator,
            Duration::from_secs(1),
        )
    }

    #[test]
    fn propagates_empty_step() {
        let expected_objects = vec![];
        let mut controller = mock_controller(expected_objects);
        controller.step().unwrap();
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
                .rotation(Radians(6.0))
                .mobility(Mobility::Movable(Velocity { x: 0, y: -1 }))
                .kind(Kind::Organism)
                .build()
                .expect("Failed to create object"),
        ];
        let mut controller = mock_controller(expected_objects);
        controller.step().unwrap();
    }
}
