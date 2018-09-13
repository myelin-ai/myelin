pub(crate) mod constant;
pub mod js;

use crate::presenter::View;
use crate::view_model::{Kind, Object, ViewModel};

pub struct CanvasView {
    context: js::CanvasRenderingContext2D,
}

impl View for CanvasView {
    fn draw_objects(&self, view_model: &ViewModel) {
        for object in &view_model.objects {
            self.draw_object(object);
        }
    }
}

impl CanvasView {
    pub fn new(canvas: &js::HTMLCanvasElement) -> Self {
        Self {
            context: canvas.get_context(constant::CONTEXT_TYPE),
        }
    }

    fn draw_object(&self, object: &Object) {
        self.context.begin_path();

        let first_vertex = &object.shape.vertices[0];
        self.context.move_to(first_vertex.x, first_vertex.y);

        for vertex in &object.shape.vertices[1..] {
            self.context.line_to(vertex.x, vertex.y);
        }

        self.context.close_path();

        let color = map_kind_to_color(&object.kind);
        self.context.set_fill_style(color);
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
