pub(crate) use self::delta_applier::{DeltaApplier, DeltaApplierError, DeltaApplierImpl};
pub(crate) use self::global_polygon_translator::{
    GlobalPolygonTranslator, GlobalPolygonTranslatorImpl,
};
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
    delta_applier: Box<dyn DeltaApplier>,
    global_polygon_translator: Box<dyn GlobalPolygonTranslator>,
}

impl Presenter for CanvasPresenter {
    fn present_delta(&mut self, _delta: ViewModelDelta) {
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
    pub(crate) fn new(
        view: Box<dyn View>,
        delta_applier: Box<dyn DeltaApplier>,
        global_polygon_translator: Box<dyn GlobalPolygonTranslator>,
    ) -> Self {
        Self {
            view,
            global_polygon_translator,
            delta_applier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_model::{self, ViewModel};
    use myelin_environment::object::{Kind, Location, Mobility, Radians};
    use myelin_environment::object_builder::PolygonBuilder;
    use myelin_visualization_core::view_model_delta::{ObjectDelta, ObjectDescriptionDelta};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::f64::consts::PI;
    use std::fmt::{self, Debug};

    #[derive(Debug)]
    struct ViewMock {
        expected_draw_objects_calls: RefCell<VecDeque<ViewModel>>,
        expected_flush_calls: RefCell<usize>,
    }

    impl ViewMock {
        fn new(
            expected_draw_objects_calls: VecDeque<ViewModel>,
            expected_flush_calls: usize,
        ) -> Self {
            Self {
                expected_draw_objects_calls: RefCell::new(expected_draw_objects_calls),
                expected_flush_calls: RefCell::new(expected_flush_calls),
            }
        }
    }

    impl View for ViewMock {
        fn draw_objects(&self, view_model: &ViewModel) {
            let expected_view_model = self
                .expected_draw_objects_calls
                .borrow_mut()
                .pop_front()
                .expect("Unexpected call to draw_objects()");

            assert_eq!(expected_view_model, *view_model);
        }

        fn flush(&self) {
            assert!(
                *self.expected_flush_calls.borrow() > 0,
                "Unexpected call to flush()"
            );
            *self.expected_flush_calls.borrow_mut() -= 1;
        }
    }

    impl Drop for ViewMock {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }

            assert_eq!(0, *self.expected_flush_calls.borrow());
        }
    }

    struct DeltaApplierMock {
        expected_calls: RefCell<VecDeque<(Box<dyn for<'a> Fn(&'a mut Snapshot)>, ViewModelDelta)>>,
    }

    impl Debug for DeltaApplierMock {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            unimplemented!();
        }
    }

    impl DeltaApplierMock {
        fn new(
            expected_calls: VecDeque<(Box<dyn for<'a> Fn(&'a mut Snapshot)>, ViewModelDelta)>,
        ) -> Self {
            Self {
                expected_calls: RefCell::new(expected_calls),
            }
        }
    }

    impl DeltaApplier for DeltaApplierMock {
        fn apply_delta(
            &self,
            snapshot: &mut Snapshot,
            view_model_delta: ViewModelDelta,
        ) -> Result<(), DeltaApplierError> {
            let (f, delta) = self
                .expected_calls
                .borrow_mut()
                .pop_front()
                .expect("Unexpected call to apply_delta()");

            assert_eq!(delta, view_model_delta);

            f(snapshot);

            Ok(())
        }
    }

    impl Drop for DeltaApplierMock {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }

            assert!(self.expected_calls.borrow().is_empty());
        }
    }

    #[derive(Debug)]
    struct GlobalPolygonTranslatorMock {
        expected_calls: RefCell<
            VecDeque<(
                business_object::Polygon,
                business_object::Position,
                view_model::Polygon,
            )>,
        >,
    }

    impl GlobalPolygonTranslatorMock {
        fn new(
            expected_calls: VecDeque<(
                business_object::Polygon,
                business_object::Position,
                view_model::Polygon,
            )>,
        ) -> Self {
            Self {
                expected_calls: RefCell::new(expected_calls),
            }
        }
    }

    impl GlobalPolygonTranslator for GlobalPolygonTranslatorMock {
        fn to_global_polygon(
            &self,
            polygon: &business_object::Polygon,
            position: &business_object::Position,
        ) -> view_model::Polygon {
            let (expected_polygon, expected_position, return_value) = self
                .expected_calls
                .borrow_mut()
                .pop_front()
                .expect("Unexpected call to .to_global_polygon()");

            assert_eq!(expected_polygon, *polygon);
            assert_eq!(expected_position, *position);

            return_value
        }
    }

    impl Drop for GlobalPolygonTranslatorMock {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }

            assert!(self.expected_calls.borrow().is_empty());
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
        let expected_view_model = ViewModel {
            objects: Vec::new(),
        };
        let view_mock = ViewMock::new(vec![expected_view_model].into(), 1);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(Default::default());
        let delta_applier_mock = DeltaApplierMock::new(Default::default());
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );
        presenter.present_delta(ViewModelDelta::new());
    }

    #[test]
    fn respects_previous_deltas() {}

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
        let view_mock = ViewMock::new(vec![expected_view_model].into(), 1);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(Default::default());
        let delta_applier_mock = DeltaApplierMock::new(Default::default());
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );
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
        let view_mock = ViewMock::new(vec![expected_view_model].into(), 1);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(Default::default());
        let delta_applier_mock = DeltaApplierMock::new(Default::default());
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );
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
        let view_mock = ViewMock::new(vec![expected_view_model].into(), 1);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(Default::default());
        let delta_applier_mock = DeltaApplierMock::new(Default::default());
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );
        // presenter.present_objects(&object_description);
        unimplemented!();
    }
}
