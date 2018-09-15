use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::CanvasView;
use myelin_environment::world::{World, WorldImpl};
use myelin_worldgen::generator::HardcodedGenerator;
use wasm_bindgen::prelude::*;

use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub fn init(canvas: &HtmlCanvasElement) -> InputHandler {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let world_factory = Box::new(|| -> Box<dyn World> { Box::new(WorldImpl::new()) });
    let worldgen = HardcodedGenerator::new(world_factory);
    let simulation = Box::new(SimulationImpl::new(presenter, &worldgen));
    InputHandler::new(simulation)
}
