use crate::js;
use crate::presenter::CanvasPresenter;
use crate::simulation::{Simulation, SimulationImpl};
use crate::view::CanvasView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EntryPoint {
    simulation: Box<Simulation>,
}

#[wasm_bindgen]
impl EntryPoint {
    pub fn new(canvas: &js::HTMLCanvasElement) -> Self {
        let view = Box::new(CanvasView::new(canvas));
        let presenter = Box::new(CanvasPresenter::new(view));
        let simulation = Box::new(SimulationImpl::new(presenter));
        Self { simulation }
    }

    pub fn start(&self) {}
}
