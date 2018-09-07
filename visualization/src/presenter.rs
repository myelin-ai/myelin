use crate::simulation::Presenter;

pub(crate) trait View {
    fn draw_bollocks(&self);
}

pub(crate) struct CanvasPresenter {
    view: Box<View>,
}

impl Presenter for CanvasPresenter {
    fn present_bollocks(&self) {
        self.view.draw_bollocks();
    }
}

impl CanvasPresenter {
    pub(crate) fn new(view: Box<View>) -> Self {
        Self { view }
    }
}
