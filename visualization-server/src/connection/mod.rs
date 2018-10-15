pub(crate) use self::websocket::*;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io::ErrorKind as IoErrorKind;

mod websocket;

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
