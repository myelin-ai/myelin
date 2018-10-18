use crate::controller::Presenter;
use crate::view_model::{self, ViewModel};
use myelin_environment::object as business_object;
use myelin_environment::Id;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::collections::HashMap;
use std::fmt;

mod delta_applier;
mod global_polygon_translator;

pub(crate) type Snapshot = HashMap<Id, business_object::ObjectDescription>;

pub(crate) trait View: fmt::Debug {
    fn draw_objects(&self, view_model: &ViewModel);
    fn flush(&self);
}

#[derive(Debug)]
pub(crate) struct CanvasPresenter {
    view: Box<dyn View>,
}

impl Presenter for CanvasPresenter {
    fn present_delta(&mut self, delta: ViewModelDelta) {
        unimplemented!();
    }
}

fn map_kind(kind: business_object::Kind) -> view_model::Kind {
    match kind {
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
    use myelin_environment::object::{Kind, Location, Mobility, Position, Radians};
    use myelin_environment::object_builder::PolygonBuilder;
    use myelin_visualization_core::view_model_delta::{ObjectDelta, ObjectDescriptionDelta};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::f64::consts::PI;

    #[derive(Debug)]
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
            if std::thread::panicking() {
                return;
            }

            assert!(*self.flush_was_called.borrow());
        }
    }

    fn view_model_delta(rotation: Radians) -> ViewModelDelta {
        hashmap! {
            42 => ObjectDelta::Updated(ObjectDescriptionDelta {
                shape: Some(
                    PolygonBuilder::new()
                        .vertex(-10, -10)
                        .vertex(10, -10)
                        .vertex(10, 10)
                        .vertex(-10, 10)
                        .build()
                        .unwrap(),
                ),
                location: Some(Location { x: 30, y: 40 }),
                rotation: Some(rotation),
                mobility: Some(Mobility::Immovable),
                kind: Some(Kind::Plant),
                sensor: None,
            }),
        }
    }

    #[test]
    fn maps_to_empty_view_model() {
        // let objects = Vec::new();
        let expected_view_model = ViewModel {
            objects: Vec::new(),
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        // presenter.present_delta(&objects);
        unimplemented!();
    }

    #[ignore]
    #[test]
    fn converts_to_global_object_with_no_orientation() {
        let object_description = [view_model_delta(Radians::default())];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex { x: 20, y: 30 },
                        view_model::Vertex { x: 40, y: 30 },
                        view_model::Vertex { x: 40, y: 50 },
                        view_model::Vertex { x: 20, y: 50 },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        // presenter.present_objects(&object_description);
        unimplemented!();
    }

    #[ignore]
    #[test]
    fn converts_to_global_object_with_pi_orientation() {
        let object_description = [view_model_delta(Radians::new(PI).unwrap())];
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
        // presenter.present_objects(&object_description);
        unimplemented!();
    }

    #[ignore]
    #[test]
    fn converts_to_global_object_with_arbitrary_orientation() {
        let object_description = [view_model_delta(Radians::new(3.0).unwrap())];
        let expected_view_model = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model::Polygon {
                    vertices: vec![
                        view_model::Vertex {
                            x: 40 - 2,
                            y: 50 + 1,
                        },
                        view_model::Vertex {
                            x: 20 - 1,
                            y: 50 - 2,
                        },
                        view_model::Vertex {
                            x: 20 + 2,
                            y: 30 - 1,
                        },
                        view_model::Vertex {
                            x: 40 + 1,
                            y: 30 + 2,
                        },
                    ],
                },
                kind: view_model::Kind::Plant,
            }],
        };
        let view_mock = ViewMock::new(expected_view_model);
        let presenter = CanvasPresenter::new(Box::new(view_mock));
        // presenter.present_objects(&object_description);
        unimplemented!();
    }
}
