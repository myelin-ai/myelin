use crate::input_handler::{InputHandler, InputHandlerImpl};
use crate::js;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::CanvasView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EntryPoint {
    input_handler: Box<InputHandler>,
}

#[wasm_bindgen]
impl EntryPoint {
    pub fn new(canvas: &js::HTMLCanvasElement) -> Self {
        let view = Box::new(CanvasView::new(canvas));
        let presenter = Box::new(CanvasPresenter::new(view));
        let simulation = Box::new(SimulationImpl::new(presenter));
        let input_handler = Box::new(InputHandlerImpl::new(simulation));
        Self { input_handler }
    }

    pub fn start(&self) {}
}

impl InputHandler for EntryPoint {
    fn on_timer(&mut self) {
        self.input_handler.on_timer();
    }
}
