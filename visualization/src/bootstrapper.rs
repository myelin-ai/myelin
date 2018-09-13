use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::js;
use crate::view::CanvasView;
use myelin_environment::world::WorldImpl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas: &js::HTMLCanvasElement) -> InputHandler {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let world = Box::new(WorldImpl::new());
    let simulation = Box::new(SimulationImpl::new(presenter, world));
    InputHandler::new(simulation)
}
