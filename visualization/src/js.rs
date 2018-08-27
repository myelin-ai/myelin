#![allow(drop_ref)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, setter = fillStyle)]
    pub fn set_fill_style(this: &CanvasRenderingContext2D, fill_style: &str);

    #[wasm_bindgen(method, js_name = fillRect)]
    pub fn fill_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = clearRect)]
    pub fn clear_rect(this: &CanvasRenderingContext2D, x: u32, y: u32, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = beginPath)]
    pub fn begin_path(this: &CanvasRenderingContext2D);

    #[wasm_bindgen(method, js_name = moveTo)]
    pub fn move_to(this: &CanvasRenderingContext2D, x: u32, y: u32);

    #[wasm_bindgen(method, js_name = lineTo)]
    pub fn line_to(this: &CanvasRenderingContext2D, x: u32, y: u32);

    #[wasm_bindgen(method)]
    pub fn fill(this: &CanvasRenderingContext2D);

    #[wasm_bindgen(method)]
    pub fn stroke(this: &CanvasRenderingContext2D);

    pub type HTMLCanvasElement;

    #[wasm_bindgen(method, js_name = getContext)]
    pub fn get_context(this: &HTMLCanvasElement, contextType: &str) -> CanvasRenderingContext2D;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &HTMLCanvasElement) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &HTMLCanvasElement) -> i32;

    pub type DOMRect;

    #[wasm_bindgen(method, getter, structural)]
    pub fn x(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn y(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn top(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn right(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn bottom(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, getter, structural)]
    pub fn left(this: &DOMRect) -> i32;

    #[wasm_bindgen(method, js_name = getBoundingClientRect)]
    pub fn get_bounding_client_rect(this: &HTMLCanvasElement) -> DOMRect;

    pub fn alert(s: &str);
}
