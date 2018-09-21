//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::controller::ControllerImpl;
use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::view::constant::SIMULATED_TIMESTEP;
use crate::view::CanvasView;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation::{simulation_impl::SimulationImpl, Simulation};
use myelin_environment::world::NphysicsWorld;
use myelin_object::{
    organism::StaticOrganism, plant::StaticPlant, terrain::StaticTerrain, water::StaticWater,
};
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
    let simulation_factory = Box::new(|| -> Box<dyn Simulation> {
        let world = Box::new(NphysicsWorld::with_timestep(SIMULATED_TIMESTEP));
        Box::new(SimulationImpl::new(world))
    });
    let object_factory = Box::new(|kind: Kind| match kind {
        Kind::Plant => ObjectBehavior::Immovable(Box::new(StaticPlant::new())),
        Kind::Organism => ObjectBehavior::Movable(Box::new(StaticOrganism::new())),
        Kind::Water => ObjectBehavior::Immovable(Box::new(StaticWater::new())),
        Kind::Terrain => ObjectBehavior::Immovable(Box::new(StaticTerrain::new())),
    });
    let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);
    let controller = Box::new(ControllerImpl::new(presenter, &worldgen));
    InputHandler::new(controller)
}
