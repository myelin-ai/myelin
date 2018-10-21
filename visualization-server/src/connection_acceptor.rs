#[cfg(test)]
pub use self::mock::*;

use crate::connection::Connection;
use crate::controller::{Client, ConnectionAcceptor};
use std::fmt::{self, Debug};
use std::sync::mpsc::Sender;

pub(crate) type ClientFactoryFn = dyn Fn() -> Box<dyn Client>;

pub(crate) struct WebsocketConnectionAcceptor {
    client_factory_fn: Box<ClientFactoryFn>,
}

impl WebsocketConnectionAcceptor {
    fn new(client_factory_fn: Box<ClientFactoryFn>) -> Self {
        Self { client_factory_fn }
    }
}

impl ConnectionAcceptor for WebsocketConnectionAcceptor {
    fn run(&mut self, sender: Sender<Connection>) {
        unimplemented!()
    }
}

impl Debug for WebsocketConnectionAcceptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(WebsocketConnectionAcceptor))
            .finish()
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    use crate::controller::{CurrentSnapshotFnFactory, Snapshot};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::Receiver;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    struct ConnectionAcceptorMock {
        connection: Option<Connection>,
        run_was_called: AtomicBool,
    }

    impl ConnectionAcceptorMock {}

    impl ConnectionAcceptor for ConnectionAcceptorMock {
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
