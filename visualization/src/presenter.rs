use crate::simulation::Presenter;
use crate::view_model::*;
use std::f64::consts::PI;

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
                    location: Location { x: 10, y: 10 },
                    orientation: Orietation { radians: PI / 2.0 },
                    shape: Shape::Rectangle(RectangleShape {
                        width: 20,
                        length: 30,
                    }),
                    kind: Kind::Organism,
                },
                Object {
                    location: Location { x: 15, y: 11 },
                    orientation: Orietation { radians: PI / 2.0 },
                    shape: Shape::Rectangle(RectangleShape {
                        width: 5,
                        length: 5,
                    }),
                    kind: Kind::Plant,
                },
                Object {
                    location: Location { x: 60, y: 20 },
                    orientation: Orietation { radians: PI },
                    shape: Shape::Rectangle(RectangleShape {
                        width: 80,
                        length: 5,
                    }),
                    kind: Kind::Terrain,
                },
                Object {
                    location: Location { x: 100, y: 170 },
                    orientation: Orietation { radians: 2.0 * PI },
                    shape: Shape::Rectangle(RectangleShape {
                        width: 20,
                        length: 30,
                    }),
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
