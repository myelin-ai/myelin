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
                            Vertex { x: 7, y: 8 },
                            Vertex { x: 5, y: 7 },
                            Vertex { x: 3, y: 5 },
                            Vertex { x: 8, y: 6 },
                        ],
                    },
                    kind: Kind::Organism,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 13, y: 11 },
                            Vertex { x: 9, y: 8 },
                            Vertex { x: 5, y: 5 },
                            Vertex { x: 9, y: 7 },
                        ],
                    },
                    kind: Kind::Plant,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 0, y: 0 },
                            Vertex { x: 10, y: 0 },
                            Vertex { x: 10, y: 4 },
                            Vertex { x: 0, y: 4 },
                        ],
                    },
                    kind: Kind::Terrain,
                },
                Object {
                    body: Polygon {
                        vertices: vec![
                            Vertex { x: 30, y: 30 },
                            Vertex { x: 35, y: 35 },
                            Vertex { x: 30, y: 40 },
                            Vertex { x: 25, y: 35 },
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
