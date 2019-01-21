pub(crate) use self::delta_applier::{DeltaApplier, DeltaApplierImpl};
pub(crate) use self::global_polygon_translator::{
    GlobalPolygonTranslator, GlobalPolygonTranslatorImpl,
};
use crate::controller::Presenter;
use crate::view::constant;
use crate::view_model;
use myelin_environment::object::Mobility;
use myelin_environment::Id;
use myelin_geometry::*;
use myelin_object_data::Kind;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

mod delta_applier;
mod global_polygon_translator;

#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
pub(crate) trait View: fmt::Debug {
    fn draw_objects(&self, objects: &[view_model::Object]);
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
    fn present_delta(&mut self, delta: ViewModelDelta) -> Result<(), Box<dyn Error>> {
        self.delta_applier
            .apply_delta(&mut self.current_snapshot, delta)?;

        let objects = map_objects(
            &self.current_snapshot,
            self.global_polygon_translator.borrow(),
        );

        self.view.flush();
        self.view.draw_objects(&objects);

        Ok(())
    }
}

pub type Snapshot = HashMap<Id, ObjectDescription>;
pub type ViewModelDelta = HashMap<Id, ObjectDelta>;

/// Describes what happened to an individual object in this
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectDelta {
    /// The object has been added to the world
    Created(ObjectDescription),
    /// At least one property of the object has changed
    Updated(ObjectDescriptionDelta),
    /// The object has been removed from the world
    Deleted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectDescription {
    /// The name of the object
    pub name: Option<String>,

    /// The object's kind
    pub kind: Kind,

    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Polygon,

    /// The global location of the center of the object
    pub location: Point,

    /// The object's rotation
    pub rotation: Radians,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Mobility,

    /// Whether the object is passable or not
    pub passable: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjectDescriptionDelta {
    /// The name of the object
    #[allow(clippy::option_option)]
    pub name: Option<Option<String>>,

    /// The object's kind
    pub kind: Option<Kind>,

    /// The vertices defining the shape of the object
    /// in relation to its [`position`]
    ///
    /// [`position`]: ./struct.ObjectDescription.html#structfield.location
    pub shape: Option<Polygon>,

    /// The current location of the object
    pub location: Option<Point>,

    /// The current rotation of the object
    pub rotation: Option<Radians>,

    /// The current velocity of the object, defined
    /// as a two dimensional vector relative to the
    /// objects center
    pub mobility: Option<Mobility>,
}

fn map_objects(
    snapshot: &Snapshot,
    global_polygon_translator: &dyn GlobalPolygonTranslator,
) -> Vec<view_model::Object> {
    snapshot
        .values()
        .map(|business_object| view_model::Object {
            shape: translate_shape_into_view_model(&business_object, global_polygon_translator),
            kind: translate_kind_into_view_model(business_object.kind),
            name_label: translate_name_into_view_model(&business_object),
        })
        .collect()
}

fn translate_shape_into_view_model(
    business_object: &ObjectDescription,
    global_polygon_translator: &dyn GlobalPolygonTranslator,
) -> view_model::Polygon {
    global_polygon_translator.to_global_polygon(
        &business_object.shape,
        business_object.location,
        business_object.rotation,
    )
}

fn translate_kind_into_view_model(kind: Kind) -> view_model::Kind {
    match kind {
        Kind::Organism => view_model::Kind::Organism,
        Kind::Plant => view_model::Kind::Plant,
        Kind::Water => view_model::Kind::Water,
        Kind::Terrain => view_model::Kind::Terrain,
    }
}

fn translate_name_into_view_model(
    business_object: &ObjectDescription,
) -> Option<view_model::Label> {
    business_object.name.clone().and_then(|name| {
        Some(view_model::Label {
            text: name,
            location: calculate_name_position(&business_object),
            font_color: String::from(constant::color::LABEL),
        })
    })
}

fn calculate_name_position(business_object: &ObjectDescription) -> view_model::Point {
    let aabb = business_object.shape.aabb();

    let top = aabb.upper_left.y;
    let horizontal_center = aabb.upper_left.x + (aabb.lower_right.x - aabb.upper_left.x) / 2.0;

    view_model::Point {
        x: business_object.location.x + horizontal_center + constant::offset::NAME_OFFSET.x,
        y: business_object.location.y + top + constant::offset::NAME_OFFSET.y,
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
    use super::delta_applier::DeltaApplierError;
    use super::*;
    use crate::presenter::global_polygon_translator::GlobalPolygonTranslatorMock;
    use crate::view_model;
    use mockiato::{partial_eq, partial_eq_owned, unordered_vec_eq};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::fmt::{self, Debug};
    use std::thread::panicking;

    struct DeltaApplierMock<'mock> {
        #[allow(clippy::type_complexity)]
        expected_calls: RefCell<
            VecDeque<(
                Box<dyn for<'a> Fn(&'a mut Snapshot) + 'mock>,
                ViewModelDelta,
            )>,
        >,
    }

    impl<'mock> Debug for DeltaApplierMock<'mock> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct(name_of_type!(DeltaApplierMock<'_>)).finish()
        }
    }

    impl<'mock> DeltaApplierMock<'mock> {
        #[allow(clippy::type_complexity)]
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
            if !panicking() {
                assert!(self.expected_calls.borrow().is_empty());
            }
        }
    }

    fn object_description() -> ObjectDescription {
        ObjectDescription {
            name: None,
            kind: Kind::Plant,
            shape: PolygonBuilder::default()
                .vertex(-10.0, -10.0)
                .vertex(10.0, -10.0)
                .vertex(10.0, 10.0)
                .vertex(-10.0, 10.0)
                .build()
                .unwrap(),
            mobility: Mobility::Immovable,
            location: Point { x: 30.0, y: 40.0 },
            rotation: Radians::default(),
            passable: false,
        }
    }

    fn object_description2() -> ObjectDescription {
        ObjectDescription {
            name: None,
            kind: Kind::Plant,
            shape: PolygonBuilder::default()
                .vertex(-20.0, -20.0)
                .vertex(20.0, -20.0)
                .vertex(20.0, 20.0)
                .vertex(-20.0, 20.0)
                .build()
                .unwrap(),
            mobility: Mobility::Immovable,
            location: Point { x: 30.0, y: 50.0 },
            rotation: Radians::default(),
            passable: false,
        }
    }

    #[test]
    fn maps_to_empty_view_model() {
        let mut view_mock = ViewMock::new();
        view_mock.expect_draw_objects(unordered_vec_eq(vec![]));
        view_mock.expect_flush();
        let global_polygon_translator = GlobalPolygonTranslatorMock::new();
        let delta_applier_mock = DeltaApplierMock::new(
            vec![(
                {
                    (box |snapshot: &mut Snapshot| {
                        assert_eq!(Snapshot::new(), *snapshot);
                    }) as Box<dyn for<'a> Fn(&'a mut Snapshot)>
                },
                ViewModelDelta::new(),
            )]
            .into(),
        );
        let mut presenter = CanvasPresenter::new(
            box view_mock,
            box delta_applier_mock,
            box global_polygon_translator,
        );
        presenter.present_delta(ViewModelDelta::new()).unwrap();
    }

    #[test]
    fn respects_previous_deltas() {
        let object_description_1 = object_description();
        let view_model_polygon_1 = view_model::Polygon {
            vertices: vec![view_model::Point { x: 1.0, y: 1.0 }],
        };
        let expected_view_model_1 = vec![view_model::Object {
            shape: view_model_polygon_1.clone(),
            kind: view_model::Kind::Plant,
            name_label: None,
        }];
        let view_model_delta_1 = hashmap! {
            12 => ObjectDelta::Created(object_description_1.clone())
        };

        let object_description_2 = object_description2();
        let view_model_polygon_2 = view_model::Polygon {
            vertices: vec![view_model::Point { x: 5.0, y: 5.0 }],
        };
        let expected_view_model_2 = vec![
            view_model::Object {
                shape: view_model_polygon_1.clone(),
                kind: view_model::Kind::Plant,
                name_label: None,
            },
            view_model::Object {
                shape: view_model_polygon_2.clone(),
                kind: view_model::Kind::Plant,
                name_label: None,
            },
        ];
        let view_model_delta_2 = hashmap! {
            45 => ObjectDelta::Created(object_description_2.clone())
        };

        let mut view_mock = ViewMock::new();
        view_mock.expect_draw_objects(unordered_vec_eq(expected_view_model_1));
        view_mock.expect_draw_objects(unordered_vec_eq(expected_view_model_2));
        view_mock.expect_flush().times(2);

        let mut global_polygon_translator = GlobalPolygonTranslatorMock::new();
        global_polygon_translator
            .expect_to_global_polygon(
                partial_eq_owned(object_description_1.shape.clone()),
                partial_eq(object_description_1.location),
                partial_eq(object_description_1.rotation),
            )
            .returns(view_model_polygon_1.clone())
            .times(2);
        global_polygon_translator
            .expect_to_global_polygon(
                partial_eq_owned(object_description_2.shape.clone()),
                partial_eq(object_description_2.location),
                partial_eq(object_description_2.rotation),
            )
            .returns(view_model_polygon_2.clone());

        let delta_applier_mock = DeltaApplierMock::new(
            vec![
                (
                    {
                        let object_description_1 = object_description_1.clone();
                        (box move |snapshot: &mut Snapshot| {
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

                        (box move |snapshot: &mut Snapshot| {
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
            box view_mock,
            box delta_applier_mock,
            box global_polygon_translator,
        );

        presenter.present_delta(view_model_delta_1).unwrap();
        presenter.present_delta(view_model_delta_2).unwrap();
    }

    #[test]
    fn calculate_name_position_works() {
        let position = calculate_name_position(&object_description());
        assert_eq!(view_model::Point { x: 30.0, y: 20.0 }, position);
    }

    #[test]
    fn calculate_name_position_works2() {
        let position = calculate_name_position(&object_description2());
        assert_eq!(view_model::Point { x: 30.0, y: 20.0 }, position);
    }
}
