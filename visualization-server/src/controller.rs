use crate::connection::Connection;
use myelin_environment::object::ObjectDescription;
use myelin_environment::Id;
use myelin_environment::Simulation;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub(crate) type Snapshot = HashMap<Id, ObjectDescription>;

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
    fn run(self);
    /// Returns the address that the [`ConnectionAcceptor`] listens on.
    fn address(&self) -> SocketAddr;
}

pub(crate) type CurrentSnapshotFn = dyn Fn() -> Snapshot + Send;

pub(crate) trait Client: Debug {
    fn run(&mut self);
}

pub(crate) type SimulationFactory = dyn Fn() -> Box<dyn Simulation>;
pub(crate) type CurrentSnapshotFnFactory = dyn Fn() -> Box<CurrentSnapshotFn>;

pub(crate) struct ControllerImpl {
    simulation_factory: Box<SimulationFactory>,
    connection_acceptor: Box<dyn ConnectionAcceptor>,
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
        unimplemented!()
    }
}

impl ControllerImpl {
    pub(crate) fn new(
        simulation_factory: Box<SimulationFactory>,
        connection_acceptor: Box<dyn ConnectionAcceptor>,
        expected_delta: Duration,
    ) -> Self {
        Self {
            simulation_factory,
            connection_acceptor,
            expected_delta,
            current_snapshot: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection_acceptor::ConnectionAcceptorMock;
    use myelin_environment::object::*;
    use myelin_environment::Simulation;
    use myelin_worldgen::WorldGenerator;
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    const EXPECTED_DELTA: Duration = Duration::from_millis((1.0f64 / 60.0f64) as u64);

    #[ignore]
    #[test]
    fn assembles_stuff() {
        let mut controller = ControllerImpl::new(
            Box::new(|| Box::new(SimulationMock::new(Vec::new()))),
            Box::new(ConnectionAcceptorMock::default()),
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
