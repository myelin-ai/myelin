//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::controller::ControllerImpl;
use crate::input_handler::InputHandler;
use crate::presenter::CanvasPresenter;
use crate::view::constant::SIMULATED_TIMESTEP;
use crate::view::CanvasView;
use myelin_environment::object::{Kind, ObjectBehavior};
use myelin_environment::simulation_impl::world::collision_filter::IgnoringCollisionFilterImpl;
use myelin_environment::simulation_impl::world::force_applier::SingleTimeForceApplierImpl;
use myelin_environment::simulation_impl::world::rotation_translator::NphysicsRotationTranslatorImpl;
use myelin_environment::simulation_impl::world::NphysicsWorld;
use myelin_environment::{simulation_impl::SimulationImpl, Simulation};
use myelin_object_behavior::Static;
use myelin_worldgen::generator::HardcodedGenerator;
use std::panic::{set_hook, PanicInfo};
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement};

fn panic_hook(info: &PanicInfo<'_>) {
    // The space works around an issue in Safari's Inspector:
    // (The issue is already resolved in Safari Technology Preview)
    // Bug Report: https://bugs.webkit.org/show_bug.cgi?id=189750
    console::error_1(&JsValue::from_str(&format!("{} ", info)));
}

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
    set_hook(box panic_hook);

    let view = box CanvasView::new(canvas);
    let presenter = box CanvasPresenter::new(view);
    let simulation_factory = box || -> Box<dyn Simulation> {
        let rotation_translator = NphysicsRotationTranslatorImpl::default();
        let force_applier = SingleTimeForceApplierImpl::default();
        let collision_filter = Arc::new(RwLock::new(IgnoringCollisionFilterImpl::default()));
        let world = box NphysicsWorld::with_timestep(
            SIMULATED_TIMESTEP,
            box rotation_translator,
            box force_applier,
            collision_filter,
        );
        box SimulationImpl::new(world)
    };
    let object_factory = box |_: Kind| -> Box<dyn ObjectBehavior> { box Static::default() };
    let worldgen = HardcodedGenerator::new(simulation_factory, object_factory);
    let controller = box ControllerImpl::new(presenter, &worldgen);
    InputHandler::new(controller)
}
