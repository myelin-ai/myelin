use crate::presenter::{
    ObjectDelta, ObjectDescription, ObjectDescriptionDelta, Snapshot, ViewModelDelta,
};
use myelin_engine::prelude::*;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DeltaApplierError {
    NonExistingObjectUpdated(Id),
}

impl Display for DeltaApplierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeltaApplierError::NonExistingObjectUpdated(id) => write!(
                f,
                "An object with id {} does not exist in snapshot, but was updated in delta",
                id
            ),
        }
    }
}

impl Error for DeltaApplierError {}

pub(crate) trait DeltaApplier: Debug {
    fn apply_delta(
        &self,
        snapshot: &mut Snapshot,
        view_model_delta: ViewModelDelta,
    ) -> Result<(), DeltaApplierError>;
}

#[derive(Debug)]
pub(crate) struct DeltaApplierImpl(PhantomData<()>);

impl DeltaApplierImpl {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl DeltaApplier for DeltaApplierImpl {
    fn apply_delta(
        &self,
        snapshot: &mut Snapshot,
        view_model_delta: ViewModelDelta,
    ) -> Result<(), DeltaApplierError> {
        for (id, object_delta) in view_model_delta {
            match object_delta {
                ObjectDelta::Created(object_description) => {
                    snapshot.insert(id, object_description);
                }
                ObjectDelta::Deleted => {
                    snapshot.remove(&id);
                }
                ObjectDelta::Updated(object_description_delta) => {
                    let object_description = snapshot
                        .get_mut(&id)
                        .ok_or_else(|| DeltaApplierError::NonExistingObjectUpdated(id))?;

                    apply_object_description_delta(object_description, object_description_delta);
                }
            }
        }

        Ok(())
    }
}

fn apply_object_description_delta(
    object_description: &mut ObjectDescription,
    object_description_delta: ObjectDescriptionDelta,
) {
    macro_rules! apply_delta {
        ($($name:ident),+) => {
            let ObjectDescriptionDelta {
                $($name),+
            } = object_description_delta;

            $(
                if let Some(value) = $name {
                    object_description.$name = value;
                }
            )+
        };
    }

    apply_delta!(name, kind, shape, location, rotation, mobility);
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use myelin_object_data::Kind;
    use std::f64::consts::PI;

    fn object_description() -> ObjectDescription {
        ObjectDescription {
            name: None,
            kind: Kind::Organism,
            height: 1.2,
            location: Point { x: 10.0, y: 20.0 },
            shape: PolygonBuilder::default()
                .vertex(-50.0, -50.0)
                .vertex(50.0, -50.0)
                .vertex(50.0, 50.0)
                .vertex(-50.0, 50.0)
                .build()
                .unwrap(),
            mobility: Mobility::Movable(Vector::default()),
            rotation: Radians::default(),
            passable: false,
        }
    }

    fn polygon() -> Polygon {
        PolygonBuilder::default()
            .vertex(-20.0, -20.0)
            .vertex(20.0, -20.0)
            .vertex(20.0, 20.0)
            .vertex(-20.0, 20.0)
            .build()
            .unwrap()
    }

    #[test]
    fn apply_delta_handles_created_object() {
        let delta_applier = DeltaApplierImpl::new();
        let mut snapshot = Snapshot::new();

        delta_applier
            .apply_delta(
                &mut snapshot,
                hashmap! {
                    12 => ObjectDelta::Created(object_description())
                },
            )
            .unwrap();

        assert_eq!(hashmap! { 12 => object_description() }, snapshot);
    }

    #[test]
    fn apply_delta_handles_deleted_object() {
        let delta_applier = DeltaApplierImpl::new();

        let mut snapshot = hashmap! {
            25 => object_description(),
            17 => object_description(),
        };

        delta_applier
            .apply_delta(
                &mut snapshot,
                hashmap! {
                    25 => ObjectDelta::Deleted,
                },
            )
            .unwrap();

        assert_eq!(
            hashmap! {
                17 => object_description(),
            },
            snapshot
        );
    }

    #[test]
    fn apply_delta_errors_if_updated_object_does_not_exist() {
        let delta_applier = DeltaApplierImpl::new();
        let mut snapshot = Snapshot::new();

        assert_eq!(
            Err(DeltaApplierError::NonExistingObjectUpdated(200)),
            delta_applier.apply_delta(
                &mut snapshot,
                hashmap! {
                    200 => ObjectDelta::Updated(ObjectDescriptionDelta {
                        location: Some(Point { x: 5.0, y: 5.0 }),
                        ..ObjectDescriptionDelta::default()
                    }),
                },
            )
        );
    }

    fn test_apply_delta_handles_update(
        object_description_delta: ObjectDescriptionDelta,
        expected_object_description: ObjectDescription,
    ) {
        let delta_applier = DeltaApplierImpl::new();

        let mut snapshot = hashmap! {
            102 => object_description(),
        };

        delta_applier
            .apply_delta(
                &mut snapshot,
                hashmap! {
                    102 => ObjectDelta::Updated(object_description_delta),
                },
            )
            .unwrap();

        assert_eq!(
            hashmap! {
                102 => expected_object_description,
            },
            snapshot
        );
    }

    #[test]
    fn apply_delta_handles_shape_update() {
        test_apply_delta_handles_update(
            ObjectDescriptionDelta {
                shape: Some(polygon()),
                ..ObjectDescriptionDelta::default()
            },
            {
                let mut object_description = object_description();
                object_description.shape = polygon();
                object_description
            },
        );
    }

    #[test]
    fn apply_delta_handles_location_update() {
        test_apply_delta_handles_update(
            ObjectDescriptionDelta {
                location: Some(Point { x: 100.0, y: 100.0 }),
                ..ObjectDescriptionDelta::default()
            },
            {
                let mut object_description = object_description();
                object_description.location = Point { x: 100.0, y: 100.0 };
                object_description
            },
        );
    }

    #[test]
    fn apply_delta_handles_rotation_update() {
        test_apply_delta_handles_update(
            ObjectDescriptionDelta {
                rotation: Some(Radians::try_new(PI).unwrap()),
                ..ObjectDescriptionDelta::default()
            },
            {
                let mut object_description = object_description();
                object_description.rotation = Radians::try_new(PI).unwrap();
                object_description
            },
        );
    }

    #[test]
    fn apply_delta_handles_mobility_update() {
        test_apply_delta_handles_update(
            ObjectDescriptionDelta {
                mobility: Some(Mobility::Movable(Vector { x: 10.0, y: 20.0 })),
                ..ObjectDescriptionDelta::default()
            },
            {
                let mut object_description = object_description();
                object_description.mobility = Mobility::Movable(Vector { x: 10.0, y: 20.0 });
                object_description
            },
        );
    }

    #[test]
    fn apply_delta_handles_kind_update() {
        test_apply_delta_handles_update(
            ObjectDescriptionDelta {
                kind: Some(Kind::Water),
                ..ObjectDescriptionDelta::default()
            },
            {
                let mut object_description = object_description();
                object_description.kind = Kind::Water;
                object_description
            },
        );
    }
}
