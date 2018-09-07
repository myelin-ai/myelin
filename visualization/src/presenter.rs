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
            objects: vec![
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 70, y: 80 },
                            Vertex { x: 50, y: 70 },
                            Vertex { x: 30, y: 50 },
                            Vertex { x: 80, y: 60 },
                        ],
                    },
                    kind: Kind::Organism,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 130, y: 110 },
                            Vertex { x: 90, y: 80 },
                            Vertex { x: 50, y: 50 },
                            Vertex { x: 90, y: 70 },
                        ],
                    },
                    kind: Kind::Plant,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 0, y: 0 },
                            Vertex { x: 100, y: 0 },
                            Vertex { x: 100, y: 40 },
                            Vertex { x: 0, y: 40 },
                        ],
                    },
                    kind: Kind::Terrain,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 300, y: 300 },
                            Vertex { x: 350, y: 350 },
                            Vertex { x: 300, y: 400 },
                            Vertex { x: 250, y: 350 },
                        ],
                    },
                    kind: Kind::Water,
                },
            ],
        };

        self.view.draw_objects(&view_model);
    }
}

impl CanvasPresenter {
    pub(crate) fn new(view: Box<View>) -> Self {
        Self { view }
    }
}
