pub(crate) mod constant;

use crate::presenter::View;
use crate::view_model::{Kind, Object, ViewModel};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn get_2d_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    const CONTEXT_ID: &str = "2d";
    const ERROR_MESSAGE: &str = "unable to get 2d context";

    let context = canvas
        .get_context(CONTEXT_ID)
        .expect(ERROR_MESSAGE)
        .expect(ERROR_MESSAGE);

    // This is safe because get_context() always returns `CanvasRenderingContext2d`
    // when the context id '2d' is passed.
    unsafe { std::mem::transmute(context) }
}

pub struct CanvasView {
    context: CanvasRenderingContext2d,
}

impl View for CanvasView {
    fn draw_objects(&self, view_model: &ViewModel) {
        for object in &view_model.objects {
            self.draw_object(object);
        }
    }
}

impl CanvasView {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        Self {
            context: get_2d_context(canvas),
        }
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

fn map_kind_to_color(kind: &Kind) -> &'static str {
    match kind {
        Kind::Organism => constant::color::ORANGE,
        Kind::Plant => constant::color::GREEN,
        Kind::Water => constant::color::BLUE,
        Kind::Terrain => constant::color::BROWN,
    }
}
