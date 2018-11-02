use crate::connection::Connection;
use myelin_environment::object::ObjectDescription;
use myelin_environment::Id;
use myelin_environment::Simulation;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::boxed::FnBox;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub(crate) type Snapshot = HashMap<Id, ObjectDescription>;
pub(crate) type ConnectionAcceptorFactoryFn =
    dyn Fn(Box<CurrentSnapshotFn>) -> Box<dyn ConnectionAcceptor> + Send + Sync;
pub(crate) type CurrentSnapshotFn = dyn Fn() -> Snapshot + Send + Sync;
pub(crate) type ThreadSpawnFn = dyn Fn(Box<dyn FnBox() + Send>) + Send + Sync;

pub(crate) trait Controller: Debug {
    fn run(&mut self);
}

pub(crate) trait Presenter: Debug {
    fn calculate_deltas(
        &self,
        visualized_snapshot: &Snapshot,
        simulation_snapshot: &Snapshot,
    ) -> ViewModelDelta;
}

pub(crate) trait ConnectionAcceptor: Debug {
    fn run(self: Box<Self>);
    /// Returns the address that the [`ConnectionAcceptor`] listens on.
    fn address(&self) -> SocketAddr;
}

pub(crate) trait Client: Debug {
    fn run(&mut self);
}

pub(crate) struct ControllerImpl {
    simulation: Box<dyn Simulation>,
    connection_acceptor_factory_fn: Arc<ConnectionAcceptorFactoryFn>,
    current_snapshot: Arc<RwLock<Snapshot>>,
    thread_spawn_fn: Box<ThreadSpawnFn>,
    expected_delta: Duration,
}

impl Debug for ControllerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControllerImpl")
            .field("expected_delta", &self.expected_delta)
            .finish()
    }
}

impl Controller for ControllerImpl {
    fn run(&mut self) {
        let current_snapshot = self.current_snapshot.clone();
        let current_snapshot_fn =
            Box::new(move || current_snapshot.read().unwrap().clone()) as Box<CurrentSnapshotFn>;
        let connection_acceptor_factory_fn = self.connection_acceptor_factory_fn.clone();
        (self.thread_spawn_fn)(Box::new(move || {
            let connection_acceptor = (connection_acceptor_factory_fn)(current_snapshot_fn);
            connection_acceptor.run();
        }));
        loop {
            self.simulation.step();
            *self.current_snapshot.write().unwrap() = self.simulation.objects();
        }
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        simulation: Box<dyn Simulation>,
        connection_acceptor_factory_fn: Arc<ConnectionAcceptorFactoryFn>,
        thread_spawn_fn: Box<ThreadSpawnFn>,
        expected_delta: Duration,
    ) -> Self {
        Self {
            simulation,
            connection_acceptor_factory_fn,
            expected_delta,
            thread_spawn_fn,
            current_snapshot: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection_acceptor::ConnectionAcceptorMock;
    use myelin_environment::object::*;
    use myelin_environment::{Id, Simulation};
    use myelin_worldgen::WorldGenerator;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    const EXPECTED_DELTA: Duration = Duration::from_millis((1.0f64 / 60.0f64) as u64);

    #[ignore]
    #[test]
    fn assembles_stuff() {
        let mut controller = ControllerImpl::new(
            Box::new(SimulationMock::new(HashMap::new())),
            Arc::new(|_| {
                Box::new(ConnectionAcceptorMock::default()) as Box<dyn ConnectionAcceptor>
            }),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.run();
    }

    fn main_thread_spawn_fn() -> Box<ThreadSpawnFn> {
        box move |function| function()
    }

    #[derive(Debug)]
    struct SimulationMock {
        step_was_called: bool,
        returned_objects: HashMap<Id, ObjectDescription>,
        objects_was_called: RefCell<bool>,
    }

    impl SimulationMock {
        fn new(returned_objects: HashMap<Id, ObjectDescription>) -> Self {
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
        fn objects(&self) -> HashMap<Id, ObjectDescription> {
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

}
