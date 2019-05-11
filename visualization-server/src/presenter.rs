use crate::controller::{Presenter, Snapshot};
use myelin_engine::prelude::*;
use myelin_visualization_core::view_model_delta::{
    ObjectDelta, ObjectDescriptionDelta, ViewModelDelta,
};
use std::collections::HashMap;
use wonderbox::autoresolvable;

#[derive(Debug, Default)]
pub(crate) struct DeltaPresenter;

#[autoresolvable]
impl DeltaPresenter {
    /// Constructs a new [`DeltaPresenter`]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Presenter for DeltaPresenter {
    fn calculate_deltas(
        &self,
        visualized_snapshot: &Snapshot,
        simulation_snapshot: &Snapshot,
    ) -> ViewModelDelta {
        let mut deltas: HashMap<_, _> = simulation_snapshot
            .iter()
            .map(|(&id, object)| {
                let delta = map_to_updated_or_created(visualized_snapshot, id, object);
                (id, delta)
            })
            .filter(|(_, delta)| match delta {
                ObjectDelta::Created(_) | ObjectDelta::Deleted => true,
                ObjectDelta::Updated(delta) => delta_contains_changes(delta),
            })
            .collect();

        deltas.extend(deleted_objects(visualized_snapshot, simulation_snapshot));

        deltas
    }
}

fn map_to_updated_or_created(
    visualized_snapshot: &Snapshot,
    id: Id,
    object: &ObjectDescription,
) -> ObjectDelta {
    if visualized_snapshot.contains_key(&id) {
        ObjectDelta::Updated(get_object_description_delta(
            visualized_snapshot.get(&id),
            object.clone(),
        ))
    } else {
        ObjectDelta::Created(object.clone())
    }
}

fn deleted_objects<'a>(
    visualized_snapshot: &'a Snapshot,
    simulation_snapshot: &'a Snapshot,
) -> impl Iterator<Item = (Id, ObjectDelta)> + 'a {
    visualized_snapshot
        .keys()
        .filter(move |id| !simulation_snapshot.contains_key(id))
        .map(|&id| (id, ObjectDelta::Deleted))
}

fn get_object_description_delta(
    first: Option<&ObjectDescription>,
    second: ObjectDescription,
) -> ObjectDescriptionDelta {
    ObjectDescriptionDelta {
        shape: get_delta(first.map(|o| &o.shape), second.shape),
        location: get_delta(first.map(|o| &o.location), second.location),
        rotation: get_delta(first.map(|o| &o.rotation), second.rotation),
        mobility: get_delta(first.map(|o| &o.mobility), second.mobility),
        associated_data: get_delta(first.map(|o| &o.associated_data), second.associated_data),
    }
}

fn get_delta<T>(first_option: Option<&T>, second: T) -> Option<T>
where
    T: PartialEq,
{
    match first_option {
        Some(first) if *first == second => None,
        _ => Some(second),
    }
}

fn delta_contains_changes(delta: &ObjectDescriptionDelta) -> bool {
    delta.shape.is_some()
        || delta.location.is_some()
        || delta.rotation.is_some()
        || delta.mobility.is_some()
        || delta.associated_data.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-10.0, -10.0)
                    .vertex(10.0, -10.0)
                    .vertex(10.0, 10.0)
                    .vertex(-10.0, 10.0)
                    .build()
                    .unwrap(),
            )
            .mobility(Mobility::Immovable)
            .location(30.0, 40.0)
            .rotation(Radians::default())
            .build()
            .unwrap()
    }

    #[test]
    fn get_delta_returns_none_if_equal() {
        assert_eq!(None, get_delta(Some(&1.0), 1.0))
    }

    #[test]
    fn get_delta_returns_second_if_not_equal() {
        assert_eq!(Some(2.0), get_delta(Some(&1.0), 2.0))
    }

    #[test]
    fn get_delta_returns_second_if_first_is_none() {
        assert_eq!(Some(1.0), get_delta(None, 1.0))
    }

    #[test]
    fn calculate_deltas_handles_deleted_object() {
        let mut first_snapshot = Snapshot::new();
        first_snapshot.insert(42, object_description());

        let second_snapshot = Snapshot::new();

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        assert_eq!(
            hashmap! {
                42 => ObjectDelta::Deleted,
            },
            delta
        );
    }

    #[test]
    fn calculate_deltas_handles_unchanged_object() {
        let mut first_snapshot = Snapshot::new();
        first_snapshot.insert(42, object_description());

        let second_snapshot = first_snapshot.clone();

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        assert_eq!(ViewModelDelta::new(), delta);
    }

    #[test]
    fn calculate_deltas_handles_updated_object() {
        let mut object = object_description();

        let mut first_snapshot = Snapshot::new();
        first_snapshot.insert(42, object.clone());

        object.location.x += 10.0;

        let mut second_snapshot = Snapshot::new();
        second_snapshot.insert(42, object.clone());

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        let expected_delta = ObjectDescriptionDelta {
            location: Some(object.location),
            ..ObjectDescriptionDelta::default()
        };

        assert_eq!(
            hashmap! {
                42 => ObjectDelta::Updated(expected_delta),
            },
            delta
        );
    }

    #[test]
    fn calculate_deltas_handles_added_object() {
        let object = object_description();

        let first_snapshot = Snapshot::new();

        let mut second_snapshot = Snapshot::new();
        second_snapshot.insert(42, object.clone());

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        let expected_object_description = ObjectBuilder::default()
            .shape(object.shape)
            .location(object.location.x, object.location.y)
            .rotation(object.rotation)
            .mobility(object.mobility)
            .associated_data(Vec::new())
            .build()
            .unwrap();

        assert_eq!(
            hashmap! {
                42 => ObjectDelta::Created(expected_object_description)
            },
            delta
        );
    }
}
