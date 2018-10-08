use crate::connection::Connection;
use crate::snapshot::{Snapshot, SnapshotSlice};
use myelin_environment::Simulation;
use std::error::Error;
use std::fmt::{self, Debug};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::Duration;

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

pub(crate) type SimulationFactory = dyn Fn() -> Box<dyn Simulation>;
pub(crate) type CurrentSnapshotFnFactory = dyn Fn() -> CurrentSnapshotFn;

pub(crate) trait ClientSpawner {
    fn accept_new_connections(
        &self,
        receiver: Receiver<Connection>,
        current_snapshot_fn_factory: Box<CurrentSnapshotFnFactory>,
    );
}

pub(crate) struct ControllerImpl {
    simulation_factory: Box<SimulationFactory>,
    connection_accepter: Box<dyn ConnectionAccepter>,
    expected_delta: Duration,
    current_snapshot: Arc<RwLock<Snapshot>>,
    client_spawner: Box<dyn ClientSpawner>,
}

impl Debug for ControllerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControllerImpl")
            .field("connection_accepter", &self.connection_accepter)
            .field("expected_delta", &self.expected_delta)
            .finish()
    }
}

impl Controller for ControllerImpl {
    fn run(&mut self) {
        unimplemented!()
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        simulation_factory: Box<SimulationFactory>,
        connection_accepter: Box<dyn ConnectionAccepter>,
        client_spawner: Box<dyn ClientSpawner>,
        expected_delta: Duration,
    ) -> Self {
        Self {
            simulation_factory,
            connection_accepter,
            expected_delta,
            client_spawner,
            current_snapshot: Default::default(),
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
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    const EXPECTED_DELTA: Duration = Duration::from_millis(1.0 / 60.0);

    #[test]
    fn assembles_stuff() {
        let controller = ControllerImpl::new(
            Box::new(|| SimulationMock::new(Vec::new())),
            Box::new(ConnectionAccepterMock::new()),
            Box::new(ClientSpawnerMock::new()),
            EXPECTED_DELTA,
        );
        controller.run();
    }

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

    struct ConnectionAccepterMock {
        connection: Connection,
        run_was_called: AtomicBool,
    }

    impl ConnectionAccepter for ConnectionAccepterMock {
        fn run(&mut self, sender: Sender<Connection>) {}
    }

    struct ClientSpawnerMock {
        expected_connection: Option<Connection>,
        accept_new_connections_was_called: AtomicBool,
    }

    impl ClientSpawnerMock {
        fn expect_connection(&mut self, connection: Connection) {
            *self.expected_connection = Some(connection)
        }
    }

    impl ClientSpawner for ClientSpawnerMock {
        fn accept_new_connections(
            &self,
            receiver: Receiver<Connection>,
            current_snapshot_fn_factory: Box<CurrentSnapshotFnFactory>,
        ) {
            accept_new_connections_was_called.store(true, Ordering::SeqCst);
            if let Some(expected_connection) = self.expected_connection {
                assert_eq!(
                    expected_connection,
                    receiver.recv().expect("Sender disconnected")
                )
            } else {
                match receiver.try_recv() {
                    Err(std::sync::mpsc::TryRecvError::Empty) => {}
                    otherwise => panic!("No connection expected, but got {:#?}", otherwise),
                }
            }
        }
    }

    impl Drop for ClientSpawnerMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }

            if self.expected_connection.is_some() {
                assert!(
                    self.accept_new_connections_was_called
                        .load(Ordering::SeqCst),
                    "accept_new_connections was not called but was expected"
                );
            }
        }
    }
}
