#[cfg(test)]
pub(crate) use self::mock::*;

use crate::connection::{Connection, Socket};
use crate::controller::{Client, ConnectionAcceptor};
use std::boxed::FnBox;
use std::fmt::{self, Debug};
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use threadpool::ThreadPool;
use uuid::Uuid;
use websocket::server::upgrade::{sync::Buffer, WsUpgrade as Request};
use websocket::server::NoTlsAcceptor;
use websocket::sync::{Client as WsClient, Server};

pub(crate) type ClientFactoryFn = dyn Fn(Connection) -> Box<dyn Client> + Send + Sync;
pub(crate) type SocketFactoryFn = dyn Fn(WsClient<TcpStream>) -> Box<dyn Socket> + Send + Sync;
pub(crate) type ThreadSpawnFn = dyn Fn(Box<dyn FnBox() + Send>) + Send + Sync;

pub(crate) struct WebsocketConnectionAcceptor {
    thread_pool: ThreadPool,
    websocket_server: Server<NoTlsAcceptor>,
    client_factory_fn: Arc<ClientFactoryFn>,
    socket_factory_fn: Arc<SocketFactoryFn>,
    thread_spawn_fn: Box<ThreadSpawnFn>,
}

impl WebsocketConnectionAcceptor {
    pub(crate) fn try_new(
        max_connections: usize,
        address: SocketAddr,
        client_factory_fn: Arc<ClientFactoryFn>,
        socket_factory_fn: Arc<SocketFactoryFn>,
        thread_spawn_fn: Box<ThreadSpawnFn>,
    ) -> Result<Self, WebsocketConnectionAcceptorError> {
        if max_connections == 0 {
            Err(WebsocketConnectionAcceptorError::NoAllowedConnectionsError)
        } else {
            Ok(Self {
                thread_pool: ThreadPool::with_name("Client spawner".to_string(), max_connections),
                websocket_server: Server::bind(address)?,
                client_factory_fn,
                socket_factory_fn,
                thread_spawn_fn,
            })
        }
    }
}

impl ConnectionAcceptor for WebsocketConnectionAcceptor {
    fn run(self) {
        for request in self.websocket_server.filter_map(Result::ok) {
            let client_factory_fn = self.client_factory_fn.clone();
            let socket_factory_fn = self.socket_factory_fn.clone();
            (self.thread_spawn_fn)(Box::new(move || {
                if should_accept(&request) {
                    if let Ok(client) = request.accept() {
                        let connection = to_connection(client, socket_factory_fn);
                        let mut client = (client_factory_fn)(connection);
                        client.run();
                    }
                }
            }))
        }
    }

    fn address(&self) -> SocketAddr {
        self.websocket_server
            .local_addr()
            .expect("Unable to get local_addr() from socket")
    }
}

fn to_connection(
    client: WsClient<TcpStream>,
    socket_factory_fn: Arc<SocketFactoryFn>,
) -> Connection {
    let id = Uuid::new_v4();
    let socket = (socket_factory_fn)(client);
    Connection { id, socket }
}

impl Debug for WebsocketConnectionAcceptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(WebsocketConnectionAcceptor))
            .finish()
    }
}

fn should_accept(_request: &Request<TcpStream, Option<Buffer>>) -> bool {
    true
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

        fn address(&self) -> SocketAddr {
            unimplemented!()
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
    use crate::connection::SocketMock;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::{self, panicking};
    use websocket::{ClientBuilder, Message};

    #[test]
    fn returns_err_on_no_allowed_connections() {
        let max_connections = 0;
        let address = localhost();
        let client_factory_fn = mock_client_factory_fn(None);
        let socket_factory_fn = mock_socket_factory_fn(Some(SocketMock::default()));
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor_result = WebsocketConnectionAcceptor::try_new(
            max_connections,
            address,
            client_factory_fn,
            socket_factory_fn,
            main_thread_spawn_fn,
        );

        match connection_acceptor_result {
            Err(WebsocketConnectionAcceptorError::NoAllowedConnectionsError) => {}
            _ => panic!("Test didn't return expected error"),
        };
    }

    #[test]
    fn accepts_connections() {
        let max_connections = 1;
        let address = localhost();
        let client_factory_fn = mock_client_factory_fn(None);
        let socket_factory_fn = mock_socket_factory_fn(Some(SocketMock::default()));
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor = WebsocketConnectionAcceptor::try_new(
            max_connections,
            address,
            client_factory_fn,
            socket_factory_fn,
            main_thread_spawn_fn,
        )
        .unwrap();

        let address = connection_acceptor.address();
        let acceptor_thread = thread::spawn(move || {
            connection_acceptor.run();
        });

        let _client = ClientBuilder::new(&format!("ws://{}", address))
            .unwrap()
            .connect_insecure()
            .unwrap();
        let result = acceptor_thread.join();
        assert!(result.is_err())
    }

    #[test]
    fn respects_max_connections() {
        let max_connections = 1;
        let address = localhost();
        let client_factory_fn = mock_client_factory_fn(None);
        let socket_factory_fn = mock_socket_factory_fn(Some(SocketMock::default()));
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor = WebsocketConnectionAcceptor::try_new(
            max_connections,
            address,
            client_factory_fn,
            socket_factory_fn,
            main_thread_spawn_fn,
        )
        .unwrap();

        let address = connection_acceptor.address();
        let acceptor_thread = thread::spawn(move || {
            connection_acceptor.run();
        });

        let first_client = ClientBuilder::new(&format!("ws://{}", address))
            .unwrap()
            .connect_insecure()
            .unwrap();

        let second_client = ClientBuilder::new(&format!("ws://{}", address))
            .unwrap()
            .connect_insecure()
            .unwrap();
    }

    fn localhost() -> SocketAddr {
        const RANDOM_PORT: u16 = 0;
        let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, RANDOM_PORT);
        SocketAddr::V4(address)
    }

    fn mock_client_factory_fn(
        expected_call: Option<(Connection, ClientMock)>,
    ) -> Arc<ClientFactoryFn> {
        Arc::new(move |connection| {
            if let Some((ref expected_connection, ref return_value)) = expected_call {
                assert_eq!(
                    *expected_connection, connection,
                    "Expected {:?}, got {:?}",
                    expected_connection, connection
                );

                Box::new(return_value.clone())
            } else {
                panic!("No call to client_factory_fn was expected")
            }
        })
    }

    fn mock_socket_factory_fn(return_value: Option<SocketMock>) -> Arc<SocketFactoryFn> {
        Arc::new(move |_| {
            if let Some(ref return_value) = return_value {
                Box::new(return_value.clone())
            } else {
                panic!("No call to socket_factory_fn was expected")
            }
        })
    }

    fn main_thread_spawn_fn() -> Box<ThreadSpawnFn> {
        Box::new(move |function| function())
    }

    #[derive(Debug, Default)]
    struct ClientMock {
        expect_run: AtomicBool,
        run_was_called: AtomicBool,
    }

    impl Clone for ClientMock {
        fn clone(&self) -> Self {
            Self {
                expect_run: AtomicBool::new(self.expect_run.load(Ordering::SeqCst)),
                run_was_called: AtomicBool::new(self.run_was_called.load(Ordering::SeqCst)),
            }
        }
    }

    impl Client for ClientMock {
        fn run(&mut self) {
            assert!(
                self.expect_run.load(Ordering::SeqCst),
                "run() was called unexpectedly"
            );
            self.run_was_called.store(true, Ordering::SeqCst);
        }
    }

    impl Drop for ClientMock {
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
