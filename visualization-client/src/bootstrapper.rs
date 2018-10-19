//! Entrypoint for the crate,
//! used to setup the entire visualization

use crate::controller::ControllerImpl;
use crate::input_handler::InputHandler;
use crate::presenter::{CanvasPresenter, DeltaApplierImpl, GlobalPolygonTranslatorImpl};
use crate::view::CanvasView;
use myelin_visualization_core::serialization::ViewModelDeserializer;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::error::Error;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[derive(Debug)]
struct DummyViewModelDeserializer {}

impl ViewModelDeserializer for DummyViewModelDeserializer {
    fn deserialize_view_model(&self, _buf: &[u8]) -> Result<ViewModelDelta, Box<dyn Error>> {
        unimplemented!();
    }
}

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
        box DummyViewModelDeserializer {},
    ))
}
