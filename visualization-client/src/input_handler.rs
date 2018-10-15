//! Functionality to communicate with the controller
//! once it's running.

use wasm_bindgen::prelude::*;

/// Struct used to signal user interaction or events.
/// Created by [`bootstrapper::init()`].
///
/// [`bootstrapper::init()`]: ../bootstrapper/fn.init.html

#[wasm_bindgen]
#[derive(Debug)]
pub struct InputHandler;

#[wasm_bindgen]
impl InputHandler {
    pub(crate) fn new() -> Self {
        InputHandler
    }

    pub fn on_message(&self, _message: &[u8]) {}
}
