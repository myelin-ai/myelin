//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::controller::ControllerImpl;
use crate::input_handler::InputHandler;
use crate::presenter::{CanvasPresenter, DeltaApplierImpl, GlobalPolygonTranslatorImpl};
use crate::view::CanvasView;
use myelin_visualization_core::serialization::BincodeDeserializer;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Initializes all components with explicit implementations
/// and returns a [`InputHandler`] that one can use to signal
/// user interaction. This function is intended to be called from
/// JavaScript or, preferably, TypeScript.
///
/// [`InputHandler`]: ../input_handler/struct.InputHandler.html
#[wasm_bindgen]
pub fn init(canvas: &HtmlCanvasElement) -> InputHandler {
    InputHandler::new(box ControllerImpl::new(
        box CanvasPresenter::new(
            box CanvasView::new(canvas),
            box DeltaApplierImpl::new(),
            box GlobalPolygonTranslatorImpl::new(),
        ),
        box BincodeDeserializer::new(),
    ))
}
