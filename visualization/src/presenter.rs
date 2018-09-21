use crate::controller::Presenter;
use crate::view_model::{self, ViewModel};
use myelin_environment::object as business_object;

pub(crate) trait View {
    fn draw_objects(&self, view_model: &ViewModel);
    fn flush(&self);
}

pub(crate) struct CanvasPresenter {
    view: Box<dyn View>,
}

impl Presenter for CanvasPresenter {
    fn present_objects(&self, objects: &[business_object::ObjectDescription]) {
        let view_model = ViewModel {
            objects: objects
                .iter()
                .map(|object| to_global_object(object))
                .collect(),
        };

        self.view.flush();
        self.view.draw_objects(&view_model);
    }
}

fn to_global_object(object: &business_object::ObjectDescription) -> view_model::Object {
    /*
    view_model::Object {
        shape: view_model::Polygon {
            vertices: object
                .shape
                .vertices
                .iter()
                .map(|vertex| view_model::Vertex {
                    x: vertex.x as u32,
                    y: vertex.y as u32,
                }).collect(),
        },
        kind: map_kind(&object.kind),
    }
    */
    unimplemented!()
}

fn map_kind(kind: &business_object::Kind) -> view_model::Kind {
    match *kind {
        business_object::Kind::Organism => view_model::Kind::Organism,
        business_object::Kind::Plant => view_model::Kind::Plant,
        business_object::Kind::Water => view_model::Kind::Water,
        business_object::Kind::Terrain => view_model::Kind::Terrain,
    }
}

impl CanvasPresenter {
    pub(crate) fn new(view: Box<dyn View>) -> Self {
        Self { view }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_model::{self, ViewModel};
    use myelin_environment::object::{Kind, Mobility, ObjectDescription, Radians};
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::cell::RefCell;
    use std::f64::consts::PI;

    struct ViewMock {
        expected_view_model: ViewModel,
        flush_was_called: RefCell<bool>,
    }

    impl ViewMock {
        fn new(expected_view_model: ViewModel) -> Self {
            Self {
                expected_view_model,
                flush_was_called: RefCell::new(false),
            }
        }
    }

    impl View for ViewMock {
        fn draw_objects(&self, view_model: &ViewModel) {
            assert_eq!(self.expected_view_model, *view_model);
        }

        fn flush(&self) {
            *self.flush_was_called.borrow_mut() = true;
        }
    }

    impl Drop for ViewMock {
        fn drop(&mut self) {
            assert!(*self.flush_was_called.borrow());
        }
    }

    fn object_description(orientation: Radians) -> ObjectDescription {
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-10, -10)
                    .vertex(10, -10)
                    .vertex(10, 10)
                    .vertex(-10, 10)
                    .build()
                    .unwrap(),
            ).velocity(Mobility::Immovable)
            .location(30, 40)
            .orientation(orientation)
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    #[test]
    fn maps_to_empty_view_model() {
        let objects = Vec::new();
        let expected_view_model = ViewModel {
            objects: Vec::new(),
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        presenter.present_objects(&objects);
    }

    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let object_description = [object_description(Radians(3.0))];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex { x: 40, y: 50 },
                        view_model::Vertex { x: 20, y: 50 },
                        view_model::Vertex { x: 20, y: 30 },
                        view_model::Vertex { x: 40, y: 30 },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        presenter.present_objects(&object_description);
    }

    #[test]
    fn converts_to_global_object_with_pi_orientation() {
        let object_description = [object_description(Radians(PI))];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex { x: 40, y: 30 },
                        view_model::Vertex { x: 40, y: 50 },
                        view_model::Vertex { x: 20, y: 50 },
                        view_model::Vertex { x: 20, y: 30 },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        presenter.present_objects(&object_description);
    }

    #[test]
    fn converts_to_global_object_with_arbitrary_orientation() {
        let object_description = [object_description(Radians(3.0))];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex {
                            x: 20 - 1,
                            y: 30 + 2,
                        },
                        view_model::Vertex {
                            x: 40 - 2,
                            y: 30 - 1,
                        },
                        view_model::Vertex {
                            x: 40 + 1,
                            y: 50 - 2,
                        },
                        view_model::Vertex {
                            x: 20 + 2,
                            y: 50 + 1,
                        },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        presenter.present_objects(&object_description);
    }
}
