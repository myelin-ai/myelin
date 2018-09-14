use crate::simulation::Simulation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputHandler {
    simulation: Box<dyn Simulation>,
}

#[wasm_bindgen]
impl InputHandler {
    pub(crate) fn new(simulation: Box<Simulation>) -> Self {
        Self { simulation }
    }

    pub fn on_timer(&mut self) {
        self.simulation.step();
    }
}
