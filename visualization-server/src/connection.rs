use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io::ErrorKind as IoErrorKind;
use websocket::client::sync::Client;
use websocket::result::WebSocketError;
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

pub(crate) struct WebsocketClient(Client<TcpStream>);

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

#[derive(Debug)]
struct WebsocketClientError(WebSocketError);

impl From<WebSocketError> for WebsocketClientError {
    fn from(error: WebSocketError) -> Self {
        WebsocketClientError(error)
    }
}

impl Error for WebsocketClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl SocketError for WebsocketClientError {
    fn is_broken_pipe(&self) -> bool {
        if let WebSocketError::IoError(err) = self.0 {
            if let IoErrorKind::BrokenPipe = err.kind() {
                return true;
            }
        };

        false
    }
}

impl Display for WebsocketClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
