use crate::connection::Connection;
use crate::snapshot::Snapshot;
use myelin_environment::object::ObjectDescription;
use myelin_environment::Id;
use myelin_environment::Simulation;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub(crate) trait Controller: Debug {
    fn run(&mut self);
}

pub(crate) trait Presenter: Debug {
    fn calculate_deltas(
        &self,
        last_objects: &HashMap<Id, ObjectDescription>,
        current_objects: &HashMap<Id, ObjectDescription>,
    ) -> ViewModelDelta;
}

pub(crate) trait ConnectionAccepter: Debug + Send {
    fn run(&mut self, sender: Sender<Connection>);
}

pub(crate) type CurrentSnapshotFn = dyn Fn() -> Snapshot + Send;

pub(crate) trait Client: Debug {
    fn run(&mut self);
}

pub(crate) type SimulationFactory = dyn Fn() -> Box<dyn Simulation>;
pub(crate) type CurrentSnapshotFnFactory = dyn Fn() -> Box<CurrentSnapshotFn>;

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
    use myelin_environment::Simulation;
    use myelin_worldgen::WorldGenerator;
    use std::cell::RefCell;
    use std::error::Error;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    const EXPECTED_DELTA: Duration = Duration::from_millis((1.0f64 / 60.0f64) as u64);

    #[test]
    fn assembles_stuff() {
        let mut controller = ControllerImpl::new(
            Box::new(|| Box::new(SimulationMock::new(Vec::new()))),
            Box::new(ConnectionAccepterMock::default()),
            Box::new(ClientSpawnerMock::default()),
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

    #[derive(Debug, Default)]
    struct PresenterMock {
        expect_calculate_deltas_and_return: Option<(
            HashMap<Id, ObjectDescription>,
            HashMap<Id, ObjectDescription>,
            ViewModelDelta,
        )>,
        calculate_deltas_was_called: RefCell<bool>,
    }
    impl PresenterMock {
        fn expect_calculate_deltas(
            &mut self,
            last_objects: HashMap<Id, ObjectDescription>,
            current_objects: HashMap<Id, ObjectDescription>,
            return_value: ViewModelDelta,
        ) {
            self.expect_calculate_deltas_and_return =
                Some((last_objects, current_objects, return_value));
        }
    }
    impl Presenter for PresenterMock {
        fn calculate_deltas(
            &self,
            visualized_objects: &HashMap<Id, ObjectDescription>,
            simulated_objects: &HashMap<Id, ObjectDescription>,
        ) -> ViewModelDelta {
            *self.calculate_deltas_was_called.borrow_mut() = true;

            if let Some((
                ref expected_visualized_objects,
                ref expected_simulated_objects,
                ref return_value,
            )) = self.expect_calculate_deltas_and_return
            {
                if *visualized_objects == *expected_visualized_objects
                    && *simulated_objects == *expected_simulated_objects
                {
                    return_value.clone()
                } else {
                    panic!(
                        "calculate_deltas() was called with {:?} and {:?}, expected {:?} and {:?}",
                        visualized_objects,
                        simulated_objects,
                        expected_visualized_objects,
                        expected_simulated_objects
                    )
                }
            } else {
                panic!("to_nphysics_rotation() was called unexpectedly")
            }
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_calculate_deltas_and_return.is_some() {
                assert!(
                    *self.calculate_deltas_was_called.borrow(),
                    "calculate_deltas() was not called, but expected"
                )
            }
        }
    }

    struct NphysicsRotationTranslatorMock {
        expect_to_nphysics_rotation_and_return: Option<(Radians, f64)>,
        expect_to_radians_and_return: Option<(f64, Radians)>,

        to_nphysics_rotation_was_called: RefCell<bool>,
        to_radians_was_called: RefCell<bool>,
    }

    impl NphysicsRotationTranslatorMock {
        fn expect_to_nphysics_rotation_and_return(
            &mut self,
            input_value: Radians,
            return_value: f64,
        ) {
            self.expect_to_nphysics_rotation_and_return = Some((input_value, return_value))
        }

        fn expect_to_radians_and_return(&mut self, input_value: f64, return_value: Radians) {
            self.expect_to_radians_and_return = Some((input_value, return_value))
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

    #[derive(Debug, Default)]
    struct ConnectionAccepterMock {
        connection: Option<Connection>,
        run_was_called: AtomicBool,
    }

    impl ConnectionAccepterMock {}

    impl ConnectionAccepter for ConnectionAccepterMock {
        fn run(&mut self, sender: Sender<Connection>) {}
    }

    #[derive(Debug, Default)]
    struct ClientSpawnerMock {
        expect_accept_new_connections: Option<(Connection, Snapshot)>,
        accept_new_connections_was_called: AtomicBool,
    }

    impl ClientSpawnerMock {
        fn expect_accept_new_connections(
            &mut self,
            connection: Connection,
            current_snapshot_fn_factory: Box<CurrentSnapshotFnFactory>,
        ) {
            let current_snapshot_fn = current_snapshot_fn_factory();
            let snapshot = current_snapshot_fn();
            self.expect_accept_new_connections = Some((connection, snapshot))
        }
    }

    impl ClientSpawner for ClientSpawnerMock {
        fn accept_new_connections(
            &self,
            receiver: Receiver<Connection>,
            current_snapshot_fn_factory: Box<CurrentSnapshotFnFactory>,
        ) {
            self.accept_new_connections_was_called
                .store(true, Ordering::SeqCst);
            if let Some((ref expected_connection, ref expected_snapshot)) =
                self.expect_accept_new_connections
            {
                let connection = receiver.recv().expect("Sender disconnected");
                assert_eq!(
                    *expected_connection, connection,
                    "accept_new_connections() received connection {:#?}, expected {:#?}",
                    connection, expected_connection
                );
                let snapshot = (current_snapshot_fn_factory)()();
                assert_eq!(
                    *expected_snapshot, snapshot,
                    "accept_new_connections() received {:#?} from current_snapshot_fn_factory, expected {:#?}",
                    snapshot, expected_snapshot
                );
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

            if self.expect_accept_new_connections.is_some() {
                assert!(
                    self.accept_new_connections_was_called
                        .load(Ordering::SeqCst),
                    "accept_new_connections() was not called but was expected"
                );
            }
        }
    }
}
