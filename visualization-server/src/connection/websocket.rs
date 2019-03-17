use super::{Socket, SocketError};
use nameof::name_of_type;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io::ErrorKind as IoErrorKind;
use std::sync::Mutex;
use websocket::client::sync::Client;
use websocket::message::Message;
use websocket::result::WebSocketError;
use websocket::stream::sync::TcpStream;

pub(crate) struct WebsocketClient(Mutex<Client<TcpStream>>);

impl WebsocketClient {
    pub(crate) fn new(inner: Client<TcpStream>) -> Self {
        Self(Mutex::new(inner))
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
            .lock()
            .unwrap()
            .send_message(&message)
            .map_err(WebsocketClientError::from)
            .map_err(|err| Box::new(err) as Box<dyn SocketError>)
    }
}

#[derive(Debug)]
struct WebsocketClientError(WebSocketError);

impl From<WebSocketError> for WebsocketClientError {
    fn from(err: WebSocketError) -> Self {
        Self(err)
    }
}

impl Error for WebsocketClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl SocketError for WebsocketClientError {
    fn is_broken_pipe(&self) -> bool {
        match &self.0 {
            WebSocketError::IoError(err) => match err.kind() {
                IoErrorKind::BrokenPipe => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Display for WebsocketClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::thread;
    use websocket::client::ClientBuilder;
    use websocket::message::OwnedMessage;
    use websocket::sync::Server;

    #[test]
    fn send_message_works() {
        const RANDOM_PORT: u16 = 0;
        const PAYLOAD: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let server = Server::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, RANDOM_PORT)).unwrap();
        let addr = server.local_addr().unwrap();

        let server_thread = thread::spawn(move || {
            let request = server.map(Result::ok).next().unwrap().unwrap();
            let client = request.accept().unwrap();
            let mut socket = WebsocketClient(Mutex::new(client));

            socket.send_message(&PAYLOAD).unwrap();
        });

        let client_thread = thread::spawn(move || {
            let mut client = ClientBuilder::new(&format!("ws://{}", addr))
                .unwrap()
                .connect_insecure()
                .unwrap();

            let message = client.incoming_messages().next().unwrap().unwrap();

            assert_eq!(OwnedMessage::Binary(PAYLOAD.to_vec()), message);
        });

        server_thread.join().unwrap();
        client_thread.join().unwrap();
    }
}
