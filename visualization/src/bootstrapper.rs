use crate::input_handler::{InputHandler, InputHandlerImpl};
use crate::js;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::CanvasView;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas: &js::HTMLCanvasElement) -> Box<InputHandler> {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let simulation = Box::new(SimulationImpl::new(presenter));
    let input_handler = Box::new(InputHandlerImpl::new(simulation));
    input_handler
}
