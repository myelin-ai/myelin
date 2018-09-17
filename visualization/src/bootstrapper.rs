//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::simulation::SimulationImpl;
use crate::view::constant::SIMULATED_TIMESTEP;
use crate::view::CanvasView;
use myelin_environment::world::{NphysicsWorld, World};
use myelin_worldgen::generator::HardcodedGenerator;
use wasm_bindgen::prelude::*;

use web_sys::HtmlCanvasElement;

/// Initializes all components with explicit implementations
/// and returns a [`InputHandler`] that one can use to signal
/// user interaction. This function is intended to be called from
/// JavaScript or, preferably, TypeScript.
/// # Examples
/// ```ts
///    import('../out/myelin_visualization').then((wasm) => {
///        const canvas = document.getElementById('visualization') as HTMLCanvasElement
///        const inputHandler = wasm.init(canvas)
///        inputHandler.on_timer()
///    }).catch((reason) => {
///        console.error(reason)
///        document.body.appendChild(document.createTextNode('Failed to load WASM'))
///        const reasonElement = document.createElement('pre')
///        reasonElement.innerText = reason
///        document.body.appendChild(reasonElement)
///    })
/// ```
///
/// [`InputHandler`]: ../input_handler/struct.InputHandler.html
#[wasm_bindgen]
pub fn init(canvas: &HtmlCanvasElement) -> InputHandler {
    let view = Box::new(CanvasView::new(canvas));
    let presenter = Box::new(CanvasPresenter::new(view));
    let world_factory = Box::new(|| -> Box<dyn World> {
        Box::new(NphysicsWorld::with_timestep(SIMULATED_TIMESTEP))
    });
    let worldgen = HardcodedGenerator::new(world_factory);
    let simulation = Box::new(SimulationImpl::new(presenter, &worldgen));
    InputHandler::new(simulation)
}
