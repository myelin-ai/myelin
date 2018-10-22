#[cfg(test)]
pub(crate) use self::mock::*;

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
    ) -> Result<Self, WebsocketConnectionAcceptorError> {
        if max_connections == 0 {
            Err(WebsocketConnectionAcceptorError::NoAllowedConnectionsError)
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

#[derive(Debug)]
pub(crate) enum WebsocketConnectionAcceptorError {
    NoAllowedConnectionsError,
    WebsocketServerError(io::Error),
}

impl From<io::Error> for WebsocketConnectionAcceptorError {
    fn from(error: io::Error) -> Self {
        WebsocketConnectionAcceptorError::WebsocketServerError(error)
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct ConnectionAcceptorMock {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::net::{Ipv6Addr, SocketAddrV6};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    #[test]
    fn returns_err_on_no_allowed_connections() {
        let max_connections = 0;
        let address = localhost();
        let client_factory_fn = mock_client_factory_fn(None);

        let connection_acceptor_result =
            WebsocketConnectionAcceptor::new(max_connections, address, client_factory_fn);

        match connection_acceptor_result {
            Err(WebsocketConnectionAcceptorError::NoAllowedConnectionsError) => {}
            _ => panic!("Test didn't return expected error"),
        };
    }

    fn localhost() -> SocketAddr {
        const RANDOM_PORT: u16 = 0;
        let address = SocketAddrV6::new(Ipv6Addr::LOCALHOST, RANDOM_PORT, 0, 0);
        SocketAddr::V6(address)
    }

    fn mock_client_factory_fn(
        expected_call: Option<(Connection, ClientMock)>,
    ) -> Arc<ClientFactoryFn> {
        Arc::new(move |connection| {
            let (expected_connection, return_value) =
                expected_call.expect("No call to client_factory_fn was expected");

            assert_eq!(
                expected_connection, connection,
                "Expected {:?}, got {:?}",
                expected_connection, connection
            );

            Box::new(return_value.clone())
        })
    }

    #[derive(Debug, Default, Clone)]
    struct ClientMock {
        expect_run: RefCell<bool>,
        run_was_called: RefCell<bool>,
    }

    impl Client for ClientMock {
        fn run(&mut self) {
            assert!(*self.expect_run.borrow(), "run() was called unexpectedly");
            *self.run_was_called.borrow_mut() = true;
        }
    }

    impl Drop for ClientMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if *self.expect_run.borrow() {
                assert!(
                    *self.run_was_called.borrow(),
                    "run() was not called, but was expected"
                );
            }
        }
    }
}
