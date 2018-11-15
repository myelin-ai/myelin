use crate::connection::Connection;
use crate::connection_acceptor::Client;
use crate::controller::{CurrentSnapshotFn, Presenter};
use crate::fixed_interval_sleeper::{FixedIntervalSleeper, FixedIntervalSleeperError};
use myelin_environment::Snapshot;
use myelin_visualization_core::serialization::ViewModelSerializer;
use std::fmt::{self, Debug};
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

    fn step_and_return_current_snapshot(&mut self, last_snapshot: &Snapshot) -> Snapshot {
        let (sleeper_result, snapshot) = sleep_for_fixed_interval!(self.interval, self.sleeper, {
            let current_snapshot = (self.current_snapshot_fn)();

            let deltas = self
                .presenter
                .calculate_deltas(last_snapshot, &current_snapshot);

            if !deltas.is_empty() {
                let serialized = self
                    .serializer
                    .serialize_view_model_delta(&deltas)
                    .expect("Failed to serialize delta");

                self.connection
                    .socket
                    .send_message(&serialized)
                    .expect("Failed to send message to client");
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

        snapshot
    }
}

impl Client for ClientHandler {
    fn run(&mut self) {
        let mut last_snapshot = Snapshot::new();
        loop {
            last_snapshot = self.step_and_return_current_snapshot(&last_snapshot);
        }
    }
}

impl Debug for ClientHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(ClientHandler))
            .field("presenter", &self.presenter)
            .field("serializer", &self.serializer)
            .field("connection", &self.connection)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::{SocketErrorMock, SocketMock};
    use crate::fixed_interval_sleeper::FixedIntervalSleeperError;
    use crate::presenter::PresenterMock;
    use myelin_environment::object::*;
    use myelin_geometry::*;
    use myelin_visualization_core::view_model_delta::{
        ObjectDelta, ObjectDescriptionDelta, ViewModelDelta,
    };
    use std::cell::RefCell;
    use std::error::Error;
    use std::fmt::Display;
    use std::thread::panicking;
    use uuid::Uuid;

    const INTERVAL: u64 = 1000 / 30;

    #[derive(Default)]
    struct FixedIntervalSleeperMock {
        expect_register_work_started_called: Option<((), ())>,
        expect_sleep_until_interval_passed_and_return:
            Option<(Duration, Result<(), FixedIntervalSleeperError>)>,

        register_work_started_was_called: RefCell<bool>,
        sleep_until_interval_passed_was_called: RefCell<bool>,
    }

    impl FixedIntervalSleeperMock {
        fn expect_register_work_started_called(&mut self) {
            self.expect_register_work_started_called = Some(((), ()));
        }

        fn expect_sleep_until_interval_passed_and_return(
            &mut self,
            interval: Duration,
            result: Result<(), FixedIntervalSleeperError>,
        ) {
            self.expect_sleep_until_interval_passed_and_return = Some((interval, result))
        }
    }

    impl FixedIntervalSleeper for FixedIntervalSleeperMock {
        fn register_work_started(&mut self) {
            *self.register_work_started_was_called.borrow_mut() = true;

            if self.expect_register_work_started_called.is_none() {
                panic!("register_work_started() was called unexpectedly")
            }
        }

        fn sleep_until_interval_passed(
            &self,
            _interval: Duration,
        ) -> Result<(), FixedIntervalSleeperError> {
            *self.sleep_until_interval_passed_was_called.borrow_mut() = true;

            if let Some((_, ref return_value)) = self.expect_sleep_until_interval_passed_and_return
            {
                return_value.clone()
            } else {
                panic!("sleep_until_interval_passed() was called unexpectedly")
            }
        }
    }

    #[test]
    fn can_be_constructed() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::default();
        sleeper.expect_register_work_started_called();
        sleeper.expect_sleep_until_interval_passed_and_return(interval, Ok(()));
        let presenter = Box::new(PresenterMock::default());
        let serializer = Box::new(SerializerMock::default());
        let socket = Box::new(SocketMock::default());
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };
        let current_snapshot_fn = Arc::new(|| Snapshot::new());
        let _client = ClientHandler::new(
            interval,
            Box::new(sleeper),
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
    }

    #[test]
    fn pipeline_is_run() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::default();
        sleeper.expect_register_work_started_called();
        sleeper.expect_sleep_until_interval_passed_and_return(interval, Ok(()));
        let mut presenter = Box::new(PresenterMock::default());
        presenter.expect_calculate_deltas(Snapshot::new(), snapshot(), delta());
        let mut serializer = Box::new(SerializerMock::default());
        let expected_payload = vec![0xFF, 0x01, 0x32];
        serializer
            .expect_serialize_view_model_delta_and_return(delta(), Ok(expected_payload.clone()));
        let mut socket = Box::new(SocketMock::default());
        socket.expect_send_message_and_return(expected_payload, Ok(()));
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(|| snapshot());
        let mut client = ClientHandler::new(
            interval,
            Box::new(sleeper),
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot);
    }

    #[test]
    fn nothing_is_sent_when_delta_is_empty() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::default();
        sleeper.expect_register_work_started_called();
        sleeper.expect_sleep_until_interval_passed_and_return(interval, Ok(()));
        let mut presenter = Box::new(PresenterMock::default());
        presenter.expect_calculate_deltas(Snapshot::new(), snapshot(), ViewModelDelta::default());
        let serializer = Box::new(SerializerMock::default());
        let socket = Box::new(SocketMock::default());
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(|| snapshot());
        let mut client = ClientHandler::new(
            interval,
            Box::new(sleeper),
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot);
    }

    #[should_panic]
    #[test]
    fn panics_on_serialization_error() {
        let interval = Duration::from_millis(INTERVAL);
        let mut sleeper = FixedIntervalSleeperMock::default();
        sleeper.expect_register_work_started_called();
        sleeper.expect_sleep_until_interval_passed_and_return(interval, Ok(()));
        let mut presenter = Box::new(PresenterMock::default());
        presenter.expect_calculate_deltas(Snapshot::new(), snapshot(), delta());
        let mut serializer = Box::new(SerializerMock::default());
        let err = ErrorMock;
        serializer.expect_serialize_view_model_delta_and_return(delta(), Err(err));
        let socket = Box::new(SocketMock::default());
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(|| snapshot());
        let mut client = ClientHandler::new(
            interval,
            Box::new(sleeper),
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
        let mut sleeper = FixedIntervalSleeperMock::default();
        sleeper.expect_register_work_started_called();
        sleeper.expect_sleep_until_interval_passed_and_return(interval, Ok(()));
        let mut presenter = Box::new(PresenterMock::default());
        presenter.expect_calculate_deltas(Snapshot::new(), snapshot(), delta());
        let mut serializer = Box::new(SerializerMock::default());
        let expected_payload = vec![0xFF, 0x01, 0x32];
        serializer
            .expect_serialize_view_model_delta_and_return(delta(), Ok(expected_payload.clone()));
        let mut socket = Box::new(SocketMock::default());
        let err = SocketErrorMock;
        socket.expect_send_message_and_return(expected_payload, Err(err));
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };

        let current_snapshot_fn = Arc::new(|| snapshot());
        let mut client = ClientHandler::new(
            interval,
            Box::new(sleeper),
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
        let last_snapshot = Snapshot::new();
        let current_snapshot = client.step_and_return_current_snapshot(&last_snapshot);
        assert_eq!(snapshot(), current_snapshot);
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
                .location(50.0, 50.0)
                .rotation(Radians::try_new(1.0).unwrap())
                .sensor(Sensor {
                    shape: PolygonBuilder::default()
                        .vertex(-2.0, -2.0)
                        .vertex(2.0, -2.0)
                        .vertex(2.0, 2.0)
                        .vertex(-2.0, 2.0)
                        .build()
                        .unwrap(),
                    location: Point::default(),
                    rotation: Radians::default(),
                })
                .mobility(Mobility::Movable(Vector { x: 3.0, y: -4.0 }))
                .kind(Kind::Plant)
                .build()
                .unwrap(),
        );
        expected_current_snapshot
    }

    fn delta() -> ViewModelDelta {
        let updated_object = ObjectDescriptionDelta {
            shape: None,
            location: Some(Point { x: 12.0, y: 32.0 }),
            rotation: None,
            mobility: None,
            kind: None,
            sensor: None,
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
                    .map_err(|mock| Box::new(mock) as Box<dyn Error>)
            } else {
                panic!("serialize_view_model_delta() was called unexpectedly")
            }
        }
    }

    impl Drop for SerializerMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_serialize_view_model_delta_and_return.is_some() {
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
