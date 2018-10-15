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
}

impl Client for ClientHandler {
    fn run(&mut self) {
        let mut last_snapshot = Snapshot::new();

        loop {
            let current_snapshot = (self.current_snapshot_fn)();

            let deltas = self
                .presenter
                .calculate_deltas(&last_snapshot, &current_snapshot);

            let serialized = self
                .serializer
                .serialize_view_model_delta(&deltas)
                .expect("Failed to serialize delta");

            self.connection
                .socket
                .send_message(&serialized)
                .expect("Failed to send message to client");

            std::thread::sleep(self.interval);

            last_snapshot = current_snapshot;
        }
    }
}

impl Debug for ClientHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientImpl")
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
    use myelin_visualization_core::view_model_delta::ViewModelDelta;
    use std::cell::RefCell;
    use std::error::Error;
    use uuid::Uuid;

    #[test]
    fn assembles_stuff() {
        let interval = Duration::from_millis(1000 / 30);
        let presenter = Box::new(PresenterMock::default());
        let serializer = Box::new(SerializerMock::default());
        let socket = Box::new(SocketMock::default());
        let connection = Connection {
            id: Uuid::new_v4(),
            socket,
        };
        let current_snapshot_fn = Box::new(|| Snapshot::new());
        let client = ClientHandler::with_interval(
            interval,
            presenter,
            serializer,
            connection,
            current_snapshot_fn,
        );
    }

    #[derive(Debug, Default)]
    struct SerializerMock {
        expect_serialize_view_model_delta_and_return:
            Option<(ViewModelDelta, Result<Vec<u8>, Box<dyn Error>>)>,

        serialize_view_model_delta_was_called: RefCell<bool>,
    }

    impl SerializerMock {
        fn expect_serialize_view_model_delta_and_return(
            &mut self,
            view_model_delta: ViewModelDelta,
            return_value: Result<Vec<u8>, Box<dyn Error>>,
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
            unimplemented!()
        }
    }

    #[derive(Debug, Default)]
    struct SocketMock {}

    impl Socket for SocketMock {
        fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
            unimplemented!()
        }
    }
}
