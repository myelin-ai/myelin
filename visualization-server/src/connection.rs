use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Connection {
    pub(crate) socket: Box<dyn Socket>,
}

pub(crate) trait Socket: Debug + Send {
    fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>>;
}

pub(crate) trait SocketError: Debug + Error + Send {
    fn is_broken_pipe(&self) -> bool;
}
