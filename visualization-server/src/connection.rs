use std::error::Error;
use std::fmt::{self, Debug};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

#[derive(Debug)]
pub(crate) struct Connection {
    pub(crate) id: usize,
    pub(crate) socket: Box<dyn Socket>,
}

pub(crate) trait Socket: Debug + Send {
    fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>>;
}

pub(crate) trait SocketError: Debug + Error + Send {
    fn is_broken_pipe(&self) -> bool;
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct WebsocketClient(Client<TcpStream>);

impl Debug for WebsocketClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(WebsocketClient)).finish()
    }
}

impl Socket for WebsocketClient {
    fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
        unimplemented!();
    }
}
