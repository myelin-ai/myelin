//! Functionality to communicate with the simulation
//! once it's running.

use crate::simulation::Simulation;
use wasm_bindgen::prelude::*;

/// Struct used to signal user interaction or events.
/// Created by [`bootstrapper::init()`].
///
/// [`bootstrapper::init()`]: ../bootstrapper/fn.init.html
#[wasm_bindgen]
pub struct InputHandler {
    simulation: Box<dyn Simulation>,
}

#[wasm_bindgen]
impl InputHandler {
    pub(crate) fn new(simulation: Box<dyn Simulation>) -> Self {
        Self { simulation }
    }

    /// Signal the simulation that a timer event has been fired,
    /// letting the simulation advance by one [`step`] and update
    /// the view
    ///
    /// [`step`]: ../../myelin_environment/world/trait.World.html#tymethod.step
    pub fn on_timer(&mut self) {
        self.simulation.step();
    }
}
