use wasm_bindgen::prelude::*;

pub(crate) const SIMULATED_TIMESTEP: f64 = 1.0 / 60.0;

pub(crate) mod color {
    pub(crate) const ORANGE: &str = "orange";
    pub(crate) const BLUE: &str = "blue";
    pub(crate) const GREEN: &str = "green";
    pub(crate) const BROWN: &str = "brown";
}

#[wasm_bindgen]
pub fn simulated_timestep() -> f64 {
    SIMULATED_TIMESTEP
}
