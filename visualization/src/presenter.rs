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
    fn present_objects(&self, objects: &[business_object::GlobalObject]) {
        let view_model = ViewModel {
            objects: objects
                .iter()
                .map(|object| self.business_object_to_view_model_object(object))
                .collect(),
        };

        self.view.flush();
        self.view.draw_objects(&view_model);
    }
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

    fn business_object_to_view_model_object(
        &self,
        object: &business_object::GlobalObject,
    ) -> view_model::Object {
        view_model::Object {
            shape: view_model::Polygon {
                vertices: object
                    .body
                    .shape
                    .vertices
                    .iter()
                    .map(|vertex| view_model::Vertex {
                        x: vertex.x,
                        y: vertex.y,
                    }).collect(),
            },
            kind: map_kind(&object.kind),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

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
    fn maps_to_correct_view_model() {
        let objects = vec![business_object::GlobalObject {
            body: business_object::Body {
                shape: business_object::Polygon {
                    vertices: vec![business_object::Vertex { x: 3, y: 15 }],
                },
                orientation: business_object::Radians(3.14),
                velocity: business_object::Velocity { x: -1, y: 34 },
            },
            kind: business_object::Kind::Organism,
        }];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![view_model::Vertex { x: 3, y: 15 }],
                },
                kind: view_model::Kind::Organism,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        presenter.present_objects(&objects);
    }
}
