use super::{Socket, SocketError};
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io::ErrorKind as IoErrorKind;
use websocket::client::sync::Client;
use websocket::message::Message;
use websocket::result::WebSocketError;
use websocket::stream::sync::TcpStream;

pub(crate) struct WebsocketClient(Client<TcpStream>);

impl WebsocketClient {
    pub(crate) fn new(inner: Client<TcpStream>) -> Self {
        WebsocketClient(inner)
    }
}

impl Debug for WebsocketClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(WebsocketClient)).finish()
    }
}

impl Socket for WebsocketClient {
    fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
        let message = Message::binary(payload);

        self.0
            .send_message(&message)
            .map_err(WebsocketClientError::from)
            .map_err(|err| Box::new(err) as Box<dyn SocketError>)
    }
}

#[derive(Debug)]
struct WebsocketClientError(WebSocketError);

impl From<WebSocketError> for WebsocketClientError {
    fn from(err: WebSocketError) -> Self {
        WebsocketClientError(err)
    }
}

impl Error for WebsocketClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl SocketError for WebsocketClientError {
    fn is_broken_pipe(&self) -> bool {
        if let WebSocketError::IoError(ref err) = &self.0 {
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
