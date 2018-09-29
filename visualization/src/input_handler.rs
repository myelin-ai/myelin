//! Functionality to communicate with the controller
//! once it's running.

use crate::controller::Controller;
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

#[wasm_bindgen]
impl InputHandler {
    pub(crate) fn new(controller: Box<dyn Controller>) -> Self {
        Self { controller }
    }

    /// Signal the controller that a timer event has been fired,
    /// letting the controller advance by one [`step`] and update
    /// the view
    ///
    /// [`step`]: ../../myelin_environment/world/trait.World.html#tymethod.step
    pub fn on_timer(&mut self) {
        self.controller.step();
    }
}
