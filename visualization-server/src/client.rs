use crate::connection::Connection;
use crate::controller::{Client, CurrentSnapshotFn, Presenter, Snapshot};
use myelin_visualization_core::serialization::ViewModelSerializer;
use std::fmt::{self, Debug};
use std::time::Duration;

pub(crate) struct ClientHandler {
    interval: Duration,
    presenter: Box<dyn Presenter>,
    serializer: Box<dyn ViewModelSerializer>,
    connection: Connection,
    current_snapshot_fn: Box<CurrentSnapshotFn>,
}

impl ClientHandler {
    pub(crate) fn with_interval(
        interval: Duration,
        presenter: Box<dyn Presenter>,
        serializer: Box<dyn ViewModelSerializer>,
        connection: Connection,
        current_snapshot_fn: Box<CurrentSnapshotFn>,
    ) -> Self {
        Self {
            interval,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        }
    }

    fn step_and_return_current_snapshot(&mut self, last_snapshot: &Snapshot) -> Snapshot {
        let current_snapshot = (self.current_snapshot_fn)();

        let deltas = self
            .presenter
            .calculate_deltas(last_snapshot, &current_snapshot);

        let serialized = self
            .serializer
            .serialize_view_model_delta(&deltas)
            .expect("Failed to serialize delta");

        self.connection
            .socket
            .send_message(&serialized)
            .expect("Failed to send message to client");

        std::thread::sleep(self.interval);

        current_snapshot
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
    use crate::connection::{Socket, SocketError};
    use crate::presenter::PresenterMock;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use myelin_visualization_core::view_model_delta::{ObjectDescriptionDelta, ViewModelDelta};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt::Display;
    use std::thread::panicking;
    use uuid::Uuid;
    const INTERVAL: u64 = 1000 / 30;

    #[test]
    fn can_be_constructed() {
        let interval = Duration::from_millis(INTERVAL);
        let presenter = Box::new(PresenterMock::default());
        let serializer = Box::new(SerializerMock::default());
        let socket = Box::new(SocketMock::default());
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };
        let current_snapshot_fn = Box::new(|| Snapshot::new());
        let _client = ClientHandler::with_interval(
            interval,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
    }

    #[test]
    fn pipeline_is_run() {
        let interval = Duration::from_millis(INTERVAL);
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

        let current_snapshot_fn = Box::new(|| snapshot());
        let mut client = ClientHandler::with_interval(
            interval,
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

        let current_snapshot_fn = Box::new(|| snapshot());
        let mut client = ClientHandler::with_interval(
            interval,
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

        let current_snapshot_fn = Box::new(|| snapshot());
        let mut client = ClientHandler::with_interval(
            interval,
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
            ObjectBuilder::new()
                .shape(
                    PolygonBuilder::new()
                        .vertex(-5, -5)
                        .vertex(5, -5)
                        .vertex(5, 5)
                        .vertex(-5, 5)
                        .build()
                        .unwrap(),
                )
                .location(50, 50)
                .rotation(Radians::new(1.0).unwrap())
                .sensor(Sensor {
                    shape: PolygonBuilder::new()
                        .vertex(-2, -2)
                        .vertex(2, -2)
                        .vertex(2, 2)
                        .vertex(-2, 2)
                        .build()
                        .unwrap(),
                    position: Position::default(),
                })
                .mobility(Mobility::Movable(Velocity { x: 3, y: -4 }))
                .kind(Kind::Plant)
                .build()
                .unwrap(),
        );
        expected_current_snapshot
    }

    fn delta() -> ViewModelDelta {
        let mut updated_objects = HashMap::new();
        updated_objects.insert(
            12,
            ObjectDescriptionDelta {
                shape: None,
                location: Some(Location { x: 12, y: 32 }),
                rotation: None,
                mobility: None,
                kind: None,
                sensor: None,
            },
        );
        ViewModelDelta {
            updated_objects,
            deleted_objects: Vec::new(),
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

    #[derive(Debug, Default)]
    struct SocketMock {
        expect_send_message_and_return: Option<(Vec<u8>, Result<(), SocketErrorMock>)>,

        send_message_was_called: RefCell<bool>,
    }

    impl SocketMock {
        fn expect_send_message_and_return(
            &mut self,
            payload: Vec<u8>,
            return_value: Result<(), SocketErrorMock>,
        ) {
            self.expect_send_message_and_return = Some((payload, return_value));
        }
    }

    impl Socket for SocketMock {
        fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
            *self.send_message_was_called.borrow_mut() = true;

            if let Some((ref expected_payload, ref return_value)) =
                self.expect_send_message_and_return
            {
                assert_eq!(
                    *expected_payload,
                    payload.to_vec(),
                    "send_message() was called with {:?}, expected {:?}",
                    payload,
                    expected_payload,
                );
                return_value
                    .clone()
                    .map_err(|mock| Box::new(mock) as Box<dyn SocketError>)
            } else {
                panic!("send_message() was called unexpectedly")
            }
        }
    }

    impl Drop for SocketMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_send_message_and_return.is_some() {
                assert!(
                    *self.send_message_was_called.borrow(),
                    "send_message() was not called, but expected"
                )
            }
        }
    }

    #[derive(Debug, Clone)]
    struct SocketErrorMock;

    impl Display for SocketErrorMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl SocketError for SocketErrorMock {
        fn is_broken_pipe(&self) -> bool {
            true
        }
    }

    impl Error for SocketErrorMock {}
}
