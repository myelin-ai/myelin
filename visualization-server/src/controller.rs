use myelin_engine::prelude::*;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::boxed::FnBox;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[cfg(test)]
use mockiato::mockable;

/// The snapshot provided by myelin-engine contains `ObjectDescription`,
/// which we are not interested in.
pub(crate) type Snapshot = HashMap<Id, ObjectDescription>;
pub(crate) type ConnectionAcceptorFactoryFn =
    dyn Fn(Arc<CurrentSnapshotFn>) -> Box<dyn ConnectionAcceptor> + Send + Sync;
pub(crate) type CurrentSnapshotFn = dyn Fn() -> Snapshot + Send + Sync;
pub(crate) type ThreadSpawnFn<'a> = dyn Fn(Box<dyn FnBox() + Send>) + Send + Sync + 'a;

pub(crate) trait Controller: Debug {
    fn run(&mut self);
}

#[cfg_attr(test, mockable)]
pub(crate) trait Presenter: Debug {
    fn calculate_deltas(
        &self,
        visualized_snapshot: &Snapshot,
        simulation_snapshot: &Snapshot,
    ) -> ViewModelDelta;
}

#[cfg_attr(test, mockable)]
pub(crate) trait ConnectionAcceptor: Debug {
    fn run(self: Box<Self>);
    /// Returns the address that the [`ConnectionAcceptor`] listens on.
    fn address(&self) -> SocketAddr;
}

pub(crate) struct ControllerImpl<'a> {
    simulation: Box<dyn Simulation + 'a>,
    connection_acceptor_factory_fn: Arc<ConnectionAcceptorFactoryFn>,
    current_snapshot: Arc<RwLock<Snapshot>>,
    thread_spawn_fn: Box<ThreadSpawnFn<'a>>,
    expected_delta: Duration,
}

impl<'a> Debug for ControllerImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(ControllerImpl<'a>))
            .field("expected_delta", &self.expected_delta)
            .finish()
    }
}

impl<'a> Controller for ControllerImpl<'a> {
    fn run(&mut self) {
        self.run_connection_acceptor();
        loop {
            self.step_simulation();
        }
    }
}

impl<'a> ControllerImpl<'a> {
    pub(crate) fn new(
        simulation: Box<dyn Simulation + 'a>,
        connection_acceptor_factory_fn: Arc<ConnectionAcceptorFactoryFn>,
        thread_spawn_fn: Box<ThreadSpawnFn<'a>>,
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

    fn run_connection_acceptor(&self) {
        let current_snapshot = self.current_snapshot.clone();
        let current_snapshot_fn =
            (Arc::new(move || current_snapshot.read().unwrap().clone())) as Arc<CurrentSnapshotFn>;
        let connection_acceptor_factory_fn = self.connection_acceptor_factory_fn.clone();
        (self.thread_spawn_fn)(box move || {
            let connection_acceptor = (connection_acceptor_factory_fn)(current_snapshot_fn);
            connection_acceptor.run();
        });
    }

    fn step_simulation(&mut self) {
        self.simulation.step();
        let current_snapshot: Snapshot = self
            .simulation
            .objects()
            .into_iter()
            .map(|object| (object.id, object.description))
            .collect();
        *self.current_snapshot.write().unwrap() = current_snapshot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    const EXPECTED_DELTA: Duration = Duration::from_millis((1.0f64 / 60.0f64) as u64);

    #[test]
    fn can_be_assembled() {
        ControllerImpl::new(
            box SimulationMock::new(),
            Arc::new(move |_| box ConnectionAcceptorMock::new() as Box<dyn ConnectionAcceptor>),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
    }

    #[test]
    fn runs_connection_acceptor() {
        let controller = ControllerImpl::new(
            box SimulationMock::new(),
            Arc::new(move |_| {
                let mut connection_acceptor = box ConnectionAcceptorMock::new();
                connection_acceptor.expect_run();
                connection_acceptor as Box<dyn ConnectionAcceptor>
            }),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.run_connection_acceptor();
    }

    #[test]
    fn steps_simulation_with_empty_snapshot() {
        let mut simulation = SimulationMock::new();
        simulation.expect_step();
        simulation.expect_objects_and_return(Vec::new());
        let mut controller = ControllerImpl::new(
            box simulation,
            Arc::new(move |_| panic!("No connection acceptor is expected to be created")),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.step_simulation();
    }

    #[test]
    fn steps_simulation_with_snapshot() {
        let mock_behavior = mock_behavior();

        let mut simulation = SimulationMock::new();
        simulation.expect_step();

        simulation.expect_objects_and_return(vec![Object {
            id: 0,
            description: object_description(),
            behavior: mock_behavior.as_ref(),
        }]);

        let mut controller = ControllerImpl::new(
            box simulation,
            Arc::new(move |_| panic!("No connection acceptor is expected to be created")),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.step_simulation();
    }

    #[test]
    fn snapshot_is_empty_before_step() {
        let simulation = SimulationMock::new();

        let current_snapshot_fn: Arc<Mutex<Option<Arc<CurrentSnapshotFn>>>> = Default::default();
        let snapshot_fn = current_snapshot_fn.clone();
        let controller = ControllerImpl::new(
            box simulation,
            Arc::new(move |current_snapshot_fn| {
                *snapshot_fn.lock().unwrap() = Some(current_snapshot_fn);
                let mut connection_acceptor = box ConnectionAcceptorMock::new();
                connection_acceptor.expect_run();
                connection_acceptor as Box<dyn ConnectionAcceptor>
            }),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.run_connection_acceptor();

        let current_snapshot_fn = current_snapshot_fn.lock().unwrap();
        let actual_snapshot = (current_snapshot_fn.as_ref().unwrap())();
        assert_eq!(HashMap::new(), actual_snapshot);
    }

    #[test]
    fn stepping_simulation_sets_snapshot_without_behavior() {
        let mock_behavior = mock_behavior();

        let mut simulation = SimulationMock::new();
        simulation.expect_step();

        let expected_snapshot = hashmap! {
           0 => object_description()
        };

        simulation.expect_objects_and_return(vec![Object {
            id: 0,
            description: object_description(),
            behavior: mock_behavior.as_ref(),
        }]);

        let current_snapshot_fn: Arc<Mutex<Option<Arc<CurrentSnapshotFn>>>> = Default::default();
        let snapshot_fn = current_snapshot_fn.clone();
        let mut controller = ControllerImpl::new(
            box simulation,
            Arc::new(move |current_snapshot_fn| {
                *snapshot_fn.lock().unwrap() = Some(current_snapshot_fn);
                let mut connection_acceptor = box ConnectionAcceptorMock::new();
                connection_acceptor.expect_run();
                connection_acceptor as Box<dyn ConnectionAcceptor>
            }),
            main_thread_spawn_fn(),
            EXPECTED_DELTA,
        );
        controller.run_connection_acceptor();
        controller.step_simulation();

        let current_snapshot_fn = current_snapshot_fn.lock().unwrap();
        let actual_snapshot = (current_snapshot_fn.as_ref().unwrap())();
        assert_eq!(expected_snapshot, actual_snapshot);
    }

    fn main_thread_spawn_fn<'a>() -> Box<ThreadSpawnFn<'a>> {
        box move |function| function()
    }

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .mobility(Mobility::Immovable)
            .location(10.0, 20.0)
            .shape(
                PolygonBuilder::default()
                    .vertex(-50.0, -50.0)
                    .vertex(50.0, -50.0)
                    .vertex(50.0, 50.0)
                    .vertex(-50.0, 50.0)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }

    fn mock_behavior<'a>() -> Box<dyn ObjectBehavior + 'a> {
        Box::new(ObjectBehaviorMock::new())
    }
}
