use super::Snapshot;
use myelin_environment::Id;
use myelin_visualization_core::view_model_delta::{ObjectDelta, ViewModelDelta};
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
        delta: ViewModelDelta,
    ) -> Result<(), DeltaApplierError>;
}

#[derive(Debug)]
pub(crate) struct DeltaApplierImpl(PhantomData<()>);

impl DeltaApplierImpl {
    pub(crate) fn new() -> Self {
        DeltaApplierImpl(PhantomData)
    }
}

impl DeltaApplier for DeltaApplierImpl {
    fn apply_delta(
        &self,
        snapshot: &mut Snapshot,
        delta: ViewModelDelta,
    ) -> Result<(), DeltaApplierError> {
        let ViewModelDelta {
            created_objects,
            deleted_objects,
            ..
        } = delta;

        snapshot.extend(created_objects.into_iter());

        deleted_objects.iter().for_each(|id| {
            snapshot.remove(id);
        });

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use myelin_environment::object::*;
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};

    fn object_description() -> ObjectDescription {
        ObjectBuilder::new()
            .kind(Kind::Organism)
            .mobility(Mobility::Immovable)
            .location(10, 20)
            .shape(
                PolygonBuilder::new()
                    .vertex(-50, -50)
                    .vertex(50, -50)
                    .vertex(50, 50)
                    .vertex(-50, 50)
                    .build()
                    .unwrap(),
            )
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
}
