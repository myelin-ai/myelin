use crate::simulation::Presenter;
use crate::view_model::*;

pub(crate) trait View {
    fn draw_objects(&self, view_model: &ViewModel);
}

pub(crate) struct CanvasPresenter {
    view: Box<View>,
}

impl Presenter for CanvasPresenter {
    fn present_bollocks(&self) {
        let view_model = ViewModel {
            objects: vec![Object {
                body: Polygon {
                    vertices: vec![
                        Vertex { x: 4, y: 5 },
                        Vertex { x: 2, y: 4 },
                        Vertex { x: 3, y: 2 },
                        Vertex { x: 5, y: 3 },
                    ],
                },
                kind: Kind::Organism,
            }],
        };

        self.view.draw_objects(&view_model);
    }
}

impl CanvasPresenter {
    pub(crate) fn new(view: Box<View>) -> Self {
        Self { view }
    }
}
