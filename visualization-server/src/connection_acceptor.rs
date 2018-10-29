#[cfg(test)]
pub(crate) use self::mock::*;

use crate::controller::{Client, ConnectionAcceptor};
use std::boxed::FnBox;
use std::fmt::{self, Debug};
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use websocket::server::upgrade::{sync::Buffer, WsUpgrade as Request};
use websocket::server::NoTlsAcceptor;
use websocket::sync::{Client as WsClient, Server};

pub(crate) type ClientFactoryFn = dyn Fn(WsClient<TcpStream>) -> Box<dyn Client> + Send + Sync;
pub(crate) type ThreadSpawnFn = dyn Fn(Box<dyn FnBox() + Send>) + Send + Sync;

pub(crate) struct WebsocketConnectionAcceptor {
    websocket_server: Server<NoTlsAcceptor>,
    client_factory_fn: Arc<ClientFactoryFn>,
    thread_spawn_fn: Box<ThreadSpawnFn>,
}

impl WebsocketConnectionAcceptor {
    pub(crate) fn try_new(
        address: SocketAddr,
        client_factory_fn: Arc<ClientFactoryFn>,
        thread_spawn_fn: Box<ThreadSpawnFn>,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            websocket_server: Server::bind(address)?,
            client_factory_fn,
            thread_spawn_fn,
        })
    }
}

impl ConnectionAcceptor for WebsocketConnectionAcceptor {
    fn run(self) {
        for request in self.websocket_server.filter_map(Result::ok) {
            let client_factory_fn = self.client_factory_fn.clone();
            (self.thread_spawn_fn)(box move || {
                if should_accept(&request) {
                    if let Ok(client_stream) = request.accept() {
                        let mut client = (client_factory_fn)(client_stream);
                        client.run();
                    }
                }
            })
        }
    }

    fn address(&self) -> SocketAddr {
        self.websocket_server
            .local_addr()
            .expect("Unable to get local_addr() from socket")
    }
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
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::{self, panicking};
    use websocket::ClientBuilder;

    #[test]
    fn accepts_connections() {
        let address = localhost();
        let mut expected_client = ClientMock::default();
        expected_client.expect_run();
        let client_factory_fn = mock_client_factory_fn(Some(expected_client));
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor =
            WebsocketConnectionAcceptor::try_new(address, client_factory_fn, main_thread_spawn_fn)
                .unwrap();

        let address = connection_acceptor.address();
        let _acceptor_thread = thread::spawn(move || {
            connection_acceptor.run();
        });

        let _client = ClientBuilder::new(&format!("ws://{}", address))
            .unwrap()
            .connect_insecure()
            .unwrap();
    }

    fn localhost() -> SocketAddr {
        const RANDOM_PORT: u16 = 0;
        let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, RANDOM_PORT);
        SocketAddr::V4(address)
    }

    fn mock_client_factory_fn(expected_call: Option<ClientMock>) -> Arc<ClientFactoryFn> {
        Arc::new(move |_client_stream| {
            if let Some(ref return_value) = expected_call {
                box return_value.clone()
            } else {
                panic!("No call to client_factory_fn was expected")
            }
        })
    }

    fn main_thread_spawn_fn() -> Box<ThreadSpawnFn> {
        box move |function| function()
    }

    #[derive(Debug, Default)]
    struct ClientMock {
        expect_run: AtomicBool,
        run_was_called: AtomicBool,
    }

    impl ClientMock {
        fn expect_run(&mut self) {
            self.expect_run.store(true, Ordering::SeqCst);
        }
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
