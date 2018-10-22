#[cfg(test)]
pub use self::mock::*;

use crate::connection::Connection;
use crate::controller::{Client, ConnectionAcceptor};
use std::fmt::{self, Debug};
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use threadpool::ThreadPool;
use websocket::server::upgrade::{sync::Buffer, WsUpgrade as Request};
use websocket::server::NoTlsAcceptor;
use websocket::sync::Server;

pub(crate) type ClientFactoryFn = dyn Fn(Connection) -> Box<dyn Client> + Send + Sync;

pub(crate) struct WebsocketConnectionAcceptor {
    thread_pool: ThreadPool,
    websocket_server: Server<NoTlsAcceptor>,
    client_factory_fn: Arc<ClientFactoryFn>,
}

impl WebsocketConnectionAcceptor {
    pub(crate) fn new(
        max_connections: usize,
        address: SocketAddr,
        client_factory_fn: Arc<ClientFactoryFn>,
    ) -> Result<Self, ConnectionAcceptorError> {
        if max_connections == 0 {
            Err(ConnectionAcceptorError::NoAllowedConnectionsError)
        } else {
            Ok(Self {
                thread_pool: ThreadPool::new(max_connections),
                websocket_server: Server::bind(address)?,
                client_factory_fn,
            })
        }
    }
}

impl ConnectionAcceptor for WebsocketConnectionAcceptor {
    fn run(self) {
        for request in self.websocket_server.filter_map(Result::ok) {
            let connection = to_connection(request);
            let client_factory_fn = self.client_factory_fn.clone();
            self.thread_pool.execute(move || {
                let mut client = (client_factory_fn)(connection);
                client.run();
            });
        }
    }
}

impl Debug for WebsocketConnectionAcceptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(WebsocketConnectionAcceptor))
            .finish()
    }
}

fn to_connection(request: Request<TcpStream, Option<Buffer>>) -> Connection {
    unimplemented!()
}

pub(crate) enum ConnectionAcceptorError {
    NoAllowedConnectionsError,
    WebsocketServerError(io::Error),
}

impl From<io::Error> for ConnectionAcceptorError {
    fn from(error: io::Error) -> Self {
        ConnectionAcceptorError::WebsocketServerError(error)
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    use crate::connection::Connection;
    use crate::controller::{CurrentSnapshotFnFactory, Snapshot};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::Receiver;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    struct ConnectionAcceptorMock {
        expect_run: AtomicBool,
        run_was_called: AtomicBool,
    }

    impl ConnectionAcceptorMock {
        pub(crate) fn expect_run(&mut self) {
            self.expect_run.store(true, Ordering::SeqCst);
        }
    }

    impl ConnectionAcceptor for ConnectionAcceptorMock {
        fn run(self) {
            assert!(
                self.expect_run.load(Ordering::SeqCst),
                "run() was called unexpectedly"
            );
            self.run_was_called.store(true, Ordering::SeqCst);
        }
    }

    impl Drop for ConnectionAcceptorMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_run.load(Ordering::SeqCst) {
                assert!(
                    self.run_was_called.load(Ordering::SeqCst),
                    "run() was not called, but was expected"
                );
            }
        }
    }
}
