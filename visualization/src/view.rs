use crate::js;
use crate::presenter::View;
use crate::view_model::{Object, ViewModel, Kind};

pub(crate) struct CanvasView {
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
    pub(crate) fn new(canvas: &js::HTMLCanvasElement) -> Self {
        Self {
            context: canvas.get_context("2d"),
        }
    }

    fn draw_object(&self, object: &Object) {
        self.context.move_to(object.body.vertices[0].x, object.body.vertices[0].y);
        self.context.begin_path();

        for vertex in &object.body.vertices[1..] {
            self.context.move_to(vertex.x, vertex.y);
        }

        let color = match object.kind {
            Kind::Organism => "orange",
            Kind::Plant => "green",
            Kind::Water => "blue",
            Kind::Terrain => "brown",
        };

        self.context.set_fill_style(color);
        self.context.fill();
    }
}
