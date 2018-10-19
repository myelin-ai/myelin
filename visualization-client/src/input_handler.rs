//! Functionality to communicate with the controller
//! once it's running.

use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// Struct used to signal user interaction or events.
/// Created by [`bootstrapper::init()`].
///
/// [`bootstrapper::init()`]: ../bootstrapper/fn.init.html

#[wasm_bindgen]
#[derive(Debug)]
pub struct InputHandler {
    controller: Box<dyn Controller>,
}

pub(crate) trait Controller: Debug {
    fn on_message(&mut self, message: &[u8]);
}

#[wasm_bindgen]
impl InputHandler {
    pub(crate) fn new(controller: Box<dyn Controller>) -> Self {
        InputHandler { controller }
    }

    pub fn on_message(&mut self, message: &[u8]) {
        self.controller.on_message(message);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::thread::panicking;

    #[derive(Debug)]
    struct ControllerMock {
        on_message_was_called: RefCell<bool>,
        expected_message: Vec<u8>,
    }

    impl ControllerMock {
        fn new(expected_message: Vec<u8>) -> Self {
            Self {
                expected_message,
                on_message_was_called: RefCell::new(false),
            }
        }
    }

    impl Controller for ControllerMock {
        fn on_message(&mut self, message: &[u8]) {
            *self.on_message_was_called.borrow_mut() = true;
            assert_eq!(self.expected_message, message);
        }
    }

    impl Drop for ControllerMock {
        fn drop(&mut self) {
            if !panicking() {
                assert!(
                    *self.on_message_was_called.borrow(),
                    "on_message() was never called, but was expected"
                );
            }
        }
    }

    #[test]
    fn on_message_is_propagated() {
        let message = vec![10, 20, 30];
        let controller = ControllerMock::new(message.clone());
        let mut input_handler = InputHandler::new(Box::new(controller));

        input_handler.on_message(&message);
    }
}
