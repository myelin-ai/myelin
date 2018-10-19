pub(crate) use self::delta_applier::{DeltaApplier, DeltaApplierError, DeltaApplierImpl};
pub(crate) use self::global_polygon_translator::{
    GlobalPolygonTranslator, GlobalPolygonTranslatorImpl,
};
use crate::controller::Presenter;
use crate::view_model::{self, ViewModel};
use myelin_environment::object as business_object;
use myelin_environment::Id;
use myelin_visualization_core::view_model_delta::ViewModelDelta;
use std::borrow::Borrow;
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
    current_snapshot: Snapshot,
}

impl Presenter for CanvasPresenter {
    fn present_delta(&mut self, delta: ViewModelDelta) {
        self.delta_applier
            .apply_delta(&mut self.current_snapshot, delta)
            .expect("Received delta was not valid");

        let objects = map_objects(
            &self.current_snapshot,
            self.global_polygon_translator.borrow(),
        );

        self.view.flush();
        self.view.draw_objects(&ViewModel { objects });
    }
}

fn map_objects(
    snapshot: &Snapshot,
    global_polygon_translator: &dyn GlobalPolygonTranslator,
) -> Vec<view_model::Object> {
    snapshot
        .values()
        .map(|business_object| {
            let shape = global_polygon_translator
                .to_global_polygon(&business_object.shape, &business_object.position);

            let kind = map_kind(business_object.kind);

            view_model::Object { shape, kind }
        })
        .collect()
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
            current_snapshot: Snapshot::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view_model::{self, ViewModel};
    use myelin_environment::object::*;
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
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

    struct DeltaApplierMock<'mock> {
        expected_calls: RefCell<
            VecDeque<(
                Box<dyn for<'a> Fn(&'a mut Snapshot) + 'mock>,
                ViewModelDelta,
            )>,
        >,
    }

    impl<'mock> Debug for DeltaApplierMock<'mock> {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            unimplemented!();
        }
    }

    impl<'mock> DeltaApplierMock<'mock> {
        fn new(
            expected_calls: VecDeque<(
                Box<dyn for<'a> Fn(&'a mut Snapshot) + 'mock>,
                ViewModelDelta,
            )>,
        ) -> Self {
            Self {
                expected_calls: RefCell::new(expected_calls),
            }
        }
    }

    impl<'mock> DeltaApplier for DeltaApplierMock<'mock> {
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

    impl<'mock> Drop for DeltaApplierMock<'mock> {
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
                .expect("Unexpected call to to_global_polygon()");

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

    fn object_description() -> ObjectDescription {
        ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-10, -10)
                    .vertex(10, -10)
                    .vertex(10, 10)
                    .vertex(-10, 10)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Immovable)
            .location(30, 40)
            .rotation(Radians::default())
            .kind(Kind::Plant)
            .build()
            .unwrap()
    }

    #[test]
    fn maps_to_empty_view_model() {
        let expected_view_model = ViewModel {
            objects: Vec::new(),
        };
        let view_mock = ViewMock::new(vec![expected_view_model].into(), 1);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(Default::default());
        let delta_applier_mock = DeltaApplierMock::new(
            vec![(
                {
                    Box::new(|snapshot: &mut Snapshot| {
                        assert_eq!(Snapshot::new(), *snapshot);
                    }) as Box<dyn for<'a> Fn(&'a mut Snapshot)>
                },
                ViewModelDelta::new(),
            )]
            .into(),
        );
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );
        presenter.present_delta(ViewModelDelta::new());
    }

    #[test]
    fn respects_previous_deltas() {
        let object_description_1 = object_description();
        let view_model_polygon_1 = view_model::Polygon {
            vertices: vec![view_model::Vertex { x: 1, y: 1 }],
        };
        let expected_view_model_1 = ViewModel {
            objects: vec![view_model::Object {
                shape: view_model_polygon_1.clone(),
                kind: view_model::Kind::Plant,
            }],
        };
        let view_model_delta_1 = hashmap! {
            12 => ObjectDelta::Created(object_description_1.clone())
        };

        let object_description_2 = object_description();
        let view_model_polygon_2 = view_model::Polygon {
            vertices: vec![view_model::Vertex { x: 5, y: 5 }],
        };
        let expected_view_model_2 = ViewModel {
            objects: vec![
                view_model::Object {
                    shape: view_model_polygon_1.clone(),
                    kind: view_model::Kind::Plant,
                },
                view_model::Object {
                    shape: view_model_polygon_2.clone(),
                    kind: view_model::Kind::Plant,
                },
            ],
        };
        let view_model_delta_2 = hashmap! {
            45 => ObjectDelta::Created(object_description_2.clone())
        };

        let view_mock = ViewMock::new(vec![expected_view_model_1, expected_view_model_2].into(), 2);
        let global_polygon_translator = GlobalPolygonTranslatorMock::new(
            vec![
                (
                    object_description_1.shape.clone(),
                    object_description_1.position.clone(),
                    view_model_polygon_1.clone(),
                ),
                (
                    object_description_1.shape.clone(),
                    object_description_1.position.clone(),
                    view_model_polygon_1.clone(),
                ),
                (
                    object_description_2.shape.clone(),
                    object_description_2.position.clone(),
                    view_model_polygon_2.clone(),
                ),
            ]
            .into(),
        );

        let delta_applier_mock = DeltaApplierMock::new(
            vec![
                (
                    {
                        let object_description_1 = object_description_1.clone();
                        Box::new(move |snapshot: &mut Snapshot| {
                            assert_eq!(Snapshot::new(), *snapshot);

                            snapshot.insert(12, object_description_1.clone());
                        }) as Box<dyn for<'a> Fn(&'a mut Snapshot)>
                    },
                    view_model_delta_1.clone(),
                ),
                (
                    {
                        let object_description_1 = object_description_1.clone();
                        let object_description_2 = object_description_2.clone();

                        Box::new(move |snapshot: &mut Snapshot| {
                            assert_eq!(hashmap! { 12 => object_description_1.clone() }, *snapshot);

                            snapshot.insert(45, object_description_2.clone());
                        }) as Box<dyn for<'a> Fn(&'a mut Snapshot)>
                    },
                    view_model_delta_2.clone(),
                ),
            ]
            .into(),
        );
        let mut presenter = CanvasPresenter::new(
            Box::new(view_mock),
            Box::new(delta_applier_mock),
            Box::new(global_polygon_translator),
        );

        presenter.present_delta(view_model_delta_1);
        presenter.present_delta(view_model_delta_2);
    }
}
