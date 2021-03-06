use crate::connection::Connection;
use crate::connection::SocketError;
use crate::connection_acceptor::Client;
use crate::controller::{CurrentSnapshotFn, Presenter, Snapshot};
use crate::fixed_interval_sleeper::{FixedIntervalSleeper, FixedIntervalSleeperError};
use log::{debug, error, warn};
use myelin_visualization_core::serialization::ViewModelSerializer;
use nameof::name_of;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct ClientHandler {
    interval: Duration,
    sleeper: Box<dyn FixedIntervalSleeper>,
    presenter: Box<dyn Presenter>,
    serializer: Box<dyn ViewModelSerializer>,
    connection: Connection,
    current_snapshot_fn: Arc<CurrentSnapshotFn>,
}

impl ClientHandler {
    pub(crate) fn new(
        interval: Duration,
        sleeper: Box<dyn FixedIntervalSleeper>,
        presenter: Box<dyn Presenter>,
        serializer: Box<dyn ViewModelSerializer>,
        connection: Connection,
        current_snapshot_fn: Arc<CurrentSnapshotFn>,
    ) -> Self {
        Self {
            interval,
            sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        }
    }

    fn step_and_return_current_snapshot(
        &mut self,
        last_snapshot: &Snapshot,
    ) -> Result<Snapshot, StepError> {
        let (sleeper_result, snapshot) = sleep_for_fixed_interval!(self.interval, self.sleeper, {
            let current_snapshot = (self.current_snapshot_fn)();

            let deltas = self
                .presenter
                .calculate_deltas(last_snapshot, &current_snapshot);

            if !deltas.is_empty() {
                let serialized = self
                    .serializer
                    .serialize_view_model_delta(&deltas)
                    .map_err(StepError::Serialization)?;

                self.connection
                    .socket
                    .send_message(&serialized)
                    .map_err(StepError::Socket)?;
            }

            current_snapshot
        });

        if let Err(error) = sleeper_result {
            match error {
                FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(_) => {
                    warn!("{}", error)
                }
            }
        }

        Ok(snapshot)
    }
}

impl Client for ClientHandler {
    fn run(&mut self) {
        let mut last_snapshot = Snapshot::new();
        loop {
            match self.step_and_return_current_snapshot(&last_snapshot) {
                Ok(snapshot) => last_snapshot = snapshot,
                Err(StepError::Socket(ref err)) if err.is_broken_pipe() => {
                    debug!("Client {} disconnected", self.connection.id);
                    break;
                }
                Err(err) => error!("{}", err),
            }
        }
    }
}

impl Debug for ClientHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type ClientHandler))
            .field(name_of!(presenter in ClientHandler), &self.presenter)
            .field(name_of!(serializer in ClientHandler), &self.serializer)
            .field(name_of!(connection in ClientHandler), &self.connection)
            .finish()
    }
}

#[derive(Debug)]
enum StepError {
    Serialization(Box<dyn Error>),
    Socket(Box<dyn SocketError>),
}

impl Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepError::Serialization(ref err) => write!(f, "Failed to serialize delta: {}", err),
            StepError::Socket(ref err) => write!(f, "Failed to send delta: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::{SocketErrorMock, SocketMock};
    use crate::controller::{PresenterMock, Snapshot};
    use crate::fixed_interval_sleeper::FixedIntervalSleeperMock;
    use maplit::hashmap;
    use myelin_engine::prelude::*;
    use myelin_object_data::{AdditionalObjectDescription, Kind};
    use myelin_visualization_core::view_model_delta::{
        ObjectDelta, ObjectDescriptionDelta, ViewModelDelta,
    };
    use std::cell::RefCell;
    use std::error::Error;
    use std::fmt::Display;
    use std::thread::panicking;
    use uuid::Uuid;

    const INTERVAL: u64 = 1000 / 30;

    #[test]
    fn can_be_constructed() {
        let interval = Duration::from_millis(INTERVAL);
        let sleeper = FixedIntervalSleeperMock::new();
        let presenter = box PresenterMock::new();
        let serializer = box SerializerMock::default();
        let socket = box SocketMock::default();
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };
        let current_snapshot_fn = Arc::new(Snapshot::new);
        let _client = ClientHandler::new(
            interval,
            box sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
    }

    #[test]
    fn pipeline_is_run() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::new();
        sleeper.expect_register_work_started();
        sleeper
            .expect_sleep_until_interval_passed(|arg| arg.partial_eq(interval))
            .returns(Ok(()));
        let mut presenter = box PresenterMock::new();
        presenter
            .expect_calculate_deltas(
                |arg| arg.partial_eq_owned(Snapshot::new()),
                |arg| arg.partial_eq_owned(snapshot()),
            )
            .returns(delta());
        let mut serializer = box SerializerMock::default();
        let expected_payload = vec![0xFF, 0x01, 0x32];
        serializer
            .expect_serialize_view_model_delta_and_return(delta(), Ok(expected_payload.clone()));
        let mut socket = box SocketMock::default();
        socket.expect_send_message_and_return(expected_payload, Ok(()));
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(snapshot);
        let mut client = ClientHandler::new(
            interval,
            box sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot.unwrap());
    }

    #[test]
    fn nothing_is_sent_when_delta_is_empty() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::new();
        sleeper.expect_register_work_started();
        sleeper
            .expect_sleep_until_interval_passed(|arg| arg.partial_eq(interval))
            .returns(Ok(()));
        let mut presenter = box PresenterMock::new();
        presenter
            .expect_calculate_deltas(
                |arg| arg.partial_eq_owned(Snapshot::new()),
                |arg| arg.partial_eq_owned(snapshot()),
            )
            .returns(ViewModelDelta::default());
        let serializer = box SerializerMock::default();
        let socket = box SocketMock::default();
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(snapshot);
        let mut client = ClientHandler::new(
            interval,
            box sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot.unwrap());
    }

    #[should_panic]
    #[test]
    fn panics_on_serialization_error() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::new();
        sleeper.expect_register_work_started();
        sleeper
            .expect_sleep_until_interval_passed(|arg| arg.partial_eq(interval))
            .returns(Ok(()));
        let mut presenter = box PresenterMock::new();
        presenter
            .expect_calculate_deltas(
                |arg| arg.partial_eq_owned(Snapshot::new()),
                |arg| arg.partial_eq_owned(snapshot()),
            )
            .returns(delta());
        let mut serializer = box SerializerMock::default();
        let err = ErrorMock;
        serializer.expect_serialize_view_model_delta_and_return(delta(), Err(err));
        let socket = box SocketMock::default();
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(snapshot);
        let mut client = ClientHandler::new(
            interval,
            box sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let _current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
    }

    #[should_panic]
    #[test]
    fn panics_on_transmission_error() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::new();
        sleeper.expect_register_work_started();
        sleeper
            .expect_sleep_until_interval_passed(|arg| arg.partial_eq(interval))
            .returns(Ok(()));
        let mut presenter = box PresenterMock::new();
        presenter
            .expect_calculate_deltas(
                |arg| arg.partial_eq_owned(Snapshot::new()),
                |arg| arg.partial_eq_owned(snapshot()),
            )
            .returns(delta());
        let mut serializer = box SerializerMock::default();
        let expected_payload = vec![0xFF, 0x01, 0x32];
        serializer
            .expect_serialize_view_model_delta_and_return(delta(), Ok(expected_payload.clone()));
        let mut socket = box SocketMock::default();
        let err = SocketErrorMock;
        socket.expect_send_message_and_return(expected_payload, Err(err));
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(snapshot);
        let mut client = ClientHandler::new(
            interval,
            box sleeper,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot.unwrap());
    }

    fn snapshot() -> Snapshot {
        let mut expected_current_snapshot = Snapshot::new();
        expected_current_snapshot.insert(
            12,
            ObjectBuilder::default()
                .shape(
                    PolygonBuilder::default()
                        .vertex(-5.0, -5.0)
                        .vertex(5.0, -5.0)
                        .vertex(5.0, 5.0)
                        .vertex(-5.0, 5.0)
                        .build()
                        .unwrap(),
                )
                .associated_data(AdditionalObjectDescription {
                    name: None,
                    kind: Kind::Plant,
                    height: 1.0,
                })
                .location(50.0, 50.0)
                .rotation(Radians::try_new(1.0).unwrap())
                .mobility(Mobility::Movable(Vector { x: 3.0, y: -4.0 }))
                .build()
                .unwrap(),
        );
        expected_current_snapshot
    }

    fn delta() -> ViewModelDelta {
        let updated_object = ObjectDescriptionDelta {
            location: Some(Point { x: 12.0, y: 32.0 }),
            ..ObjectDescriptionDelta::default()
        };

        hashmap! {
            12 => ObjectDelta::Updated(updated_object)
        }
    }

    #[derive(Debug, Default)]
    struct SerializerMock {
        expect_serialize_view_model_delta_and_return:
            Option<(ViewModelDelta, Result<Vec<u8>, ErrorMock>)>,

        serialize_view_model_delta_was_called: RefCell<bool>,
    }

    impl SerializerMock {
        fn expect_serialize_view_model_delta_and_return(
            &mut self,
            view_model_delta: ViewModelDelta,
            return_value: Result<Vec<u8>, ErrorMock>,
        ) {
            self.expect_serialize_view_model_delta_and_return =
                Some((view_model_delta, return_value));
        }
    }

    impl ViewModelSerializer for SerializerMock {
        fn serialize_view_model_delta(
            &self,
            view_model_delta: &ViewModelDelta,
        ) -> Result<Vec<u8>, Box<dyn Error>> {
            *self.serialize_view_model_delta_was_called.borrow_mut() = true;

            if let Some((ref expected_view_model_delta, ref return_value)) =
                self.expect_serialize_view_model_delta_and_return
            {
                assert_eq!(
                    *expected_view_model_delta, *view_model_delta,
                    "serialize_view_model_delta() was called with {:?}, expected {:?}",
                    view_model_delta, expected_view_model_delta,
                );
                return_value
                    .clone()
                    .map_err(|mock| box mock as Box<dyn Error>)
            } else {
                panic!("serialize_view_model_delta() was called unexpectedly")
            }
        }
    }

    impl Drop for SerializerMock {
        fn drop(&mut self) {
            if !panicking() && self.expect_serialize_view_model_delta_and_return.is_some() {
                assert!(
                    *self.serialize_view_model_delta_was_called.borrow(),
                    "serialize_view_model_delta() was not called, but expected"
                )
            }
        }
    }

    #[derive(Debug, Clone)]
    struct ErrorMock;

    impl Display for ErrorMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl Error for ErrorMock {}
}
