pub(crate) use self::websocket::*;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io::ErrorKind as IoErrorKind;
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
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct SocketMock {
        expect_send_message_and_return: Option<(Vec<u8>, Result<(), SocketErrorMock>)>,
        send_message_was_called: AtomicBool,
    }

    impl SocketMock {
        pub(crate) fn expect_send_message_and_return(
            &mut self,
            payload: Vec<u8>,
            return_value: Result<(), SocketErrorMock>,
        ) {
            self.expect_send_message_and_return = Some((payload, return_value));
        }
    }

    impl Socket for SocketMock {
        fn send_message(&mut self, payload: &[u8]) -> Result<(), Box<dyn SocketError>> {
            self.send_message_was_called.store(true, Ordering::SeqCst);

            if let Some((ref expected_payload, ref return_value)) =
                self.expect_send_message_and_return
            {
                if &payload.to_vec() == expected_payload {
                    return_value
                        .clone()
                        .map_err(|err| Box::new(err) as Box<dyn SocketError>)
                } else {
                    panic!(
                        "send_message() was called with {:?}, expected {:?}",
                        payload, expected_payload,
                    )
                }
            } else {
                panic!("send_message() was called unexpectedly")
            }
        }
    }

    impl Drop for SocketMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_send_message_and_return.is_some() {
                assert!(
                    self.send_message_was_called.load(Ordering::SeqCst),
                    "send_message() was not called, but expected"
                )
            }
        }
    }

    #[derive(Debug, Default)]
    pub(crate) struct SocketErrorMock {
        expect_is_broken_pipe_and_return: Option<bool>,
        is_broken_pipe_was_called: AtomicBool,
    }

    impl SocketErrorMock {
        pub(crate) fn expect_is_broken_pipe_and_return(&mut self, return_value: bool) {
            self.expect_is_broken_pipe_and_return = Some(return_value);
        }
    }

    impl SocketError for SocketErrorMock {
        fn is_broken_pipe(&self) -> bool {
            self.is_broken_pipe_was_called.store(true, Ordering::SeqCst);

            if let Some(ref return_value) = self.expect_is_broken_pipe_and_return {
                return_value.clone()
            } else {
                panic!("is_broken_pipe() was called unexpectedly")
            }
        }
    }

    impl Display for SocketErrorMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let msg = if let Some(return_value) = self.expect_is_broken_pipe_and_return {
                format!("is_broken_pipe() expected, will return {}", return_value)
            } else {
                "is_broken_pipe() not expected".to_string()
            };
            write!(f, "{}", &msg)
        }
    }

    impl Error for SocketErrorMock {}

    impl Drop for SocketErrorMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_is_broken_pipe_and_return.is_some() {
                assert!(
                    self.is_broken_pipe_was_called.load(Ordering::SeqCst),
                    "is_broken_pipe() was not called, but expected"
                )
            }
        }
    }
}
