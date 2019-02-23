use crate::controller::{ConnectionAcceptor, CurrentSnapshotFn};
use nameof::name_of_type;
use std::boxed::FnBox;
use std::fmt::{self, Debug};
use std::io;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use websocket::server::upgrade::{sync::Buffer, WsUpgrade as Request};
use websocket::server::NoTlsAcceptor;
use websocket::sync::{Client as WsClient, Server};

pub(crate) trait Client: Debug {
    fn run(&mut self);
}
pub(crate) type ClientFactoryFn =
    dyn Fn(WsClient<TcpStream>, Arc<CurrentSnapshotFn>) -> Box<dyn Client> + Send + Sync;
pub(crate) type ThreadSpawnFn = dyn Fn(Box<dyn FnBox() + Send>) + Send + Sync;

pub(crate) struct WebsocketConnectionAcceptor {
    websocket_server: Server<NoTlsAcceptor>,
    client_factory_fn: Arc<ClientFactoryFn>,
    thread_spawn_fn: Box<ThreadSpawnFn>,
    current_snapshot_fn: Arc<CurrentSnapshotFn>,
}

impl WebsocketConnectionAcceptor {
    pub(crate) fn try_new(
        address: SocketAddr,
        client_factory_fn: Arc<ClientFactoryFn>,
        thread_spawn_fn: Box<ThreadSpawnFn>,
        current_snapshot_fn: Arc<CurrentSnapshotFn>,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            websocket_server: Server::bind(address)?,
            client_factory_fn,
            thread_spawn_fn,
            current_snapshot_fn,
        })
    }
}

impl ConnectionAcceptor for WebsocketConnectionAcceptor {
    fn run(self: Box<Self>) {
        for request in self.websocket_server.filter_map(Result::ok) {
            let client_factory_fn = self.client_factory_fn.clone();
            let current_snapshot_fn = self.current_snapshot_fn.clone();
            (self.thread_spawn_fn)(box move || {
                if should_accept(&request) {
                    if let Ok(mut client_stream) = request.accept() {
                        client_stream.recv_message().unwrap();
                        let mut client = (client_factory_fn)(client_stream, current_snapshot_fn);
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
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::{self, panicking};
    use websocket::message::Message;
    use websocket::ClientBuilder;

    const RANDOM_PORT: u16 = 0;

    #[test]
    fn address_returns_correct_socket_address() {
        let address = localhost();
        let client_factory_fn = mock_client_factory_fn(None);
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor = WebsocketConnectionAcceptor::try_new(
            address,
            client_factory_fn,
            main_thread_spawn_fn,
            Arc::new(|| panic!("current_snapshot_fn was not expected to be called")),
        )
        .unwrap();

        let local_addr = connection_acceptor.address();

        assert_eq!(address.ip(), local_addr.ip());
        assert!(local_addr.port() != RANDOM_PORT);
    }

    #[test]
    fn accepts_connections() {
        let address = localhost();
        let mut expected_client = ClientMock::default();
        expected_client.expect_run();
        let client_factory_fn = mock_client_factory_fn(Some(expected_client));
        let main_thread_spawn_fn = main_thread_spawn_fn();

        let connection_acceptor = box WebsocketConnectionAcceptor::try_new(
            address,
            client_factory_fn,
            main_thread_spawn_fn,
            Arc::new(|| panic!("current_snapshot_fn was not expected to be called")),
        )
        .unwrap();

        let address = connection_acceptor.address();
        let _acceptor_thread = thread::spawn(move || {
            connection_acceptor.run();
        });

        let mut client = ClientBuilder::new(&format!("ws://{}", address))
            .unwrap()
            .connect_insecure()
            .unwrap();

        client.send_message(&Message::binary(&[] as &[u8])).unwrap();
    }

    fn localhost() -> SocketAddr {
        let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, RANDOM_PORT);
        SocketAddr::V4(address)
    }

    fn mock_client_factory_fn(expected_call: Option<ClientMock>) -> Arc<ClientFactoryFn> {
        Arc::new(move |_client_stream, _current_snapshot_fn| {
            let return_value = &expected_call
                .clone()
                .expect("No call to client_factory_fn was expected");

            box return_value.clone()
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
            if !panicking() && self.expect_run.load(Ordering::SeqCst) {
                assert!(
                    self.run_was_called.load(Ordering::SeqCst),
                    "run() was not called, but was expected"
                );
            }
        }
    }
}
