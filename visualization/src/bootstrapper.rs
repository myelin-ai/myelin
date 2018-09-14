use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::js;
use crate::view::CanvasView;
use myelin_environment::world::{World, WorldImpl};
use myelin_worldgen::generator::HardcodedGenerator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(canvas: &js::HTMLCanvasElement) -> InputHandler {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let world_factory = Box::new(|| -> Box<dyn World> { Box::new(WorldImpl::new()) });
    let worldgen = Box::new(HardcodedGenerator::new(world_factory));
    let simulation = Box::new(SimulationImpl::new(presenter, worldgen));
    InputHandler::new(simulation)
}
