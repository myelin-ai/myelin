//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::controller::ControllerImpl;
use crate::input_handler::InputHandler;
use crate::presenter::{CanvasPresenter, DeltaApplierImpl, GlobalPolygonTranslatorImpl};
use crate::view::CanvasView;
use myelin_visualization_core::serialization::BincodeDeserializer;
use std::panic::{set_hook, PanicInfo};
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement};

fn panic_hook(info: &PanicInfo<'_>) {
    console::error_1(&JsValue::from_str(&format!("{}", info)));
}

/// Initializes all components with explicit implementations
/// and returns a [`InputHandler`] that one can use to signal
/// user interaction. This function is intended to be called from
/// JavaScript or, preferably, TypeScript.
///
/// [`InputHandler`]: ./struct.InputHandler.html
#[wasm_bindgen]
pub fn init(canvas: &HtmlCanvasElement) -> InputHandler {
    set_hook(box panic_hook);

    InputHandler::new(box ControllerImpl::new(
        box CanvasPresenter::new(
            box CanvasView::new(canvas),
            box DeltaApplierImpl::new(),
            box GlobalPolygonTranslatorImpl::new(),
        ),
        box BincodeDeserializer::default(),
    ))
}
