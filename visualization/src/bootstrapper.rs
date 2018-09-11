use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::js;
use crate::view::CanvasView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas: &js::HTMLCanvasElement) -> InputHandler {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let simulation = Box::new(SimulationImpl::new(presenter));
    InputHandler::new(simulation)
}
