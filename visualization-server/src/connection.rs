pub(crate) use self::websocket::*;
use std::error::Error;
use std::fmt::Debug;
use uuid::Uuid;

mod websocket;

#[cfg(test)]
pub(crate) use self::mock::*;

#[derive(Debug)]
pub(crate) struct Connection {
    pub(crate) id: Uuid,
    pub(crate) socket: Box<dyn Socket>,
}

pub(crate) trait Socket: Debug + Send + Sync {
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

#[cfg(test)]
mod mock {
    use super::*;
    use std::fmt::{self, Display};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct SocketMock {
        #[allow(clippy::type_complexity)]
        expect_send_message_and_return: Mutex<Option<(Vec<u8>, Result<(), SocketErrorMock>)>>,
        send_message_was_called: AtomicBool,
    }

    impl SocketMock {
        pub(crate) fn expect_send_message_and_return(
            &mut self,
            payload: Vec<u8>,
            return_value: Result<(), SocketErrorMock>,
        ) {
            self.expect_send_message_and_return = Mutex::new(Some((payload, return_value)));
        }
    }

    impl Socket for SocketMock {
        fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
            self.send_message_was_called.store(true, Ordering::SeqCst);

            if let Some((ref expected_payload, ref return_value)) =
                *self.expect_send_message_and_return.lock().unwrap()
            {
                assert_eq!(
                    *expected_payload,
                    payload.to_vec(),
                    "send_message() was called with {:?}, expected {:?}",
                    payload,
                    expected_payload,
                );
                return_value
                    .clone()
                    .map_err(|mock| box mock as Box<dyn SocketError>)
            } else {
                panic!("send_message() was called unexpectedly")
            }
        }
    }

    impl Drop for SocketMock {
        fn drop(&mut self) {
            if !panicking()
                && self
                    .expect_send_message_and_return
                    .lock()
                    .unwrap()
                    .is_some()
            {
                assert!(
                    self.send_message_was_called.load(Ordering::SeqCst),
                    "send_message() was not called, but expected"
                )
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct SocketErrorMock;

    impl Display for SocketErrorMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "")
        }
    }

    impl SocketError for SocketErrorMock {
        fn is_broken_pipe(&self) -> bool {
            true
        }
    }

    impl Error for SocketErrorMock {}
}
