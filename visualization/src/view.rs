use crate::js;
use crate::presenter::View;
use crate::view_model::{Object, ViewModel};

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
        self.context.set_fill_style("aquamarine");
        self.context.fill_rect(0, 0, 20, 20);
    }
}
