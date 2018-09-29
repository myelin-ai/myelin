//! Internal module containing the DOM manipulation.
pub mod constant;

use crate::presenter::View;
use crate::view_model::{Kind, Object, ViewModel};
use std::fmt;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

/// The view object component that manipulates the DOM.
#[derive(Debug)]
pub(crate) struct CanvasView {
    context: CanvasRenderingContext2d,
}

impl View for CanvasView {
    fn draw_objects(&self, view_model: &ViewModel) {
        for object in &view_model.objects {
            self.draw_object(object);
        }
    }

    fn flush(&self) {
        let canvas = self
            .context
            .canvas()
            .expect("No association with a <canvas> element");
        self.context
            .clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    }
}

impl CanvasView {
    pub(crate) fn new(canvas: &HtmlCanvasElement) -> Self {
        let context = get_2d_context(canvas);

        adjust_canvas_to_device_pixel_ratio(canvas, &context);

        Self { context }
    }

    fn draw_object(&self, object: &Object) {
        self.context.begin_path();

        let first_vertex = &object.shape.vertices[0];
        self.context
            .move_to(f64::from(first_vertex.x), f64::from(first_vertex.y));

        for vertex in &object.shape.vertices[1..] {
            self.context
                .line_to(f64::from(vertex.x), f64::from(vertex.y));
        }

        self.context.close_path();

        let color = map_kind_to_color(&object.kind);
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill();
    }
}

fn get_2d_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    const CONTEXT_ID: &str = "2d";
    const ERROR_MESSAGE: &str = "unable to get 2d context";

    let context = canvas
        .get_context(CONTEXT_ID)
        .expect(ERROR_MESSAGE)
        .expect(ERROR_MESSAGE);

    context
        .dyn_into()
        .expect("unable to cast context into CanvasRenderingContext2d")
}

fn adjust_canvas_to_device_pixel_ratio(
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
) {
    let window = web_sys::window().expect("No window available");
    let native_pixel_ratio = window.device_pixel_ratio();
    let pixel_ratio = native_pixel_ratio.round() as u32;
    let width = canvas.width();
    let height = canvas.height();

    canvas.set_width(width * pixel_ratio);
    canvas.set_height(height * pixel_ratio);

    context
        .scale(native_pixel_ratio, native_pixel_ratio)
        .expect("Failed to scale canvas");

    let element: &HtmlElement = canvas.as_ref();

    element
        .style()
        .set_property("width", &format!("{}", Pixels(width)))
        .expect("Failed to set css width");

    element
        .style()
        .set_property("height", &format!("{}", Pixels(height)))
        .expect("Failed to set css height");
}

struct Pixels(u32);

impl fmt::Display for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

fn map_kind_to_color(kind: &Kind) -> &'static str {
    match kind {
        Kind::Organism => constant::color::ORANGE,
        Kind::Plant => constant::color::GREEN,
        Kind::Water => constant::color::BLUE,
        Kind::Terrain => constant::color::BROWN,
    }
}
