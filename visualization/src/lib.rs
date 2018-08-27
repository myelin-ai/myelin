extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod js;

#[wasm_bindgen]
pub struct EntryPoint {
    context: js::CanvasRenderingContext2D,
}

#[wasm_bindgen]
impl EntryPoint {
    pub fn new(canvas: js::HTMLCanvasElement) -> Self {
        Self {
            context: canvas.get_context("2d"),
        }
    }

    pub fn start(&self) {
        self.context.set_fill_style("aquamarine");
        self.context.fill_rect(0, 0, 20, 20);
    }
}
