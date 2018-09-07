use crate::js;
use crate::presenter::View;

pub(crate) struct CanvasView {
    context: js::CanvasRenderingContext2D,
}

impl View for CanvasView {
    fn draw_bollocks(&self) {
        self.context.set_fill_style("aquamarine");
        self.context.fill_rect(0, 0, 20, 20);
    }
}

impl CanvasView {
    pub(crate) fn new(canvas: &js::HTMLCanvasElement) -> Self {
        Self {
            context: canvas.get_context("2d"),
        }
    }
}
