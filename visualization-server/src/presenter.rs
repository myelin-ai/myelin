use crate::controller::{Presenter, Snapshot};
use myelin_environment::object::ObjectDescription;
use myelin_visualization_core::view_model_delta::{ObjectDescriptionDelta, ViewModelDelta};

#[derive(Debug, Default)]
pub(crate) struct DeltaPresenter;

fn get_object_description_delta(
    first: Option<&ObjectDescription>,
    second: ObjectDescription,
) -> ObjectDescriptionDelta {
    ObjectDescriptionDelta {
        shape: get_delta(first.map(|o| &o.shape), second.shape),
        position: get_delta(first.map(|o| &o.position), second.position),
        mobility: get_delta(first.map(|o| &o.mobility), second.mobility),
        kind: get_delta(first.map(|o| &o.kind), second.kind),
        sensor: get_delta(first.map(|o| &o.sensor), second.sensor),
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

impl Presenter for DeltaPresenter {
    fn calculate_deltas(
        &self,
        visualized_snapshot: &Snapshot,
        simulation_snapshot: &Snapshot,
    ) -> ViewModelDelta {
        let deleted_objects = visualized_snapshot
            .keys()
            .filter(|id| !simulation_snapshot.contains_key(id))
            .map(|&id| id)
            .collect();

        let updated_objects = simulation_snapshot
            .iter()
            .map(|(&id, object_description)| {
                (
                    id,
                    get_object_description_delta(
                        visualized_snapshot.get(&id),
                        object_description.clone(),
                    ),
                )
            })
            .filter(|(_, object_description)| {
                object_description.shape.is_some()
                    || object_description.position.is_some()
                    || object_description.mobility.is_some()
                    || object_description.kind.is_some()
                    || object_description.sensor.is_some()
            })
            .collect();

        ViewModelDelta {
            updated_objects,
            deleted_objects,
        }
    }
}

impl DeltaPresenter {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{Kind, Mobility, ObjectDescription, Radians};
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};

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

        assert_eq!(0, delta.updated_objects.len());
        assert_eq!(1, delta.deleted_objects.len());
        assert_eq!(42, delta.deleted_objects[0]);
    }

    #[test]
    fn calculate_deltas_handles_unchanged_object() {
        let mut first_snapshot = Snapshot::new();
        first_snapshot.insert(42, object_description());

        let second_snapshot = first_snapshot.clone();

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        assert_eq!(0, delta.updated_objects.len());
        assert_eq!(0, delta.deleted_objects.len());
    }

    #[test]
    fn calculate_deltas_handles_updated_object() {
        let mut object = object_description();

        let mut first_snapshot = Snapshot::new();
        first_snapshot.insert(42, object.clone());

        object.position.location.x += 10;

        let mut second_snapshot = Snapshot::new();
        second_snapshot.insert(42, object.clone());

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        assert_eq!(1, delta.updated_objects.len());
        assert_eq!(0, delta.deleted_objects.len());

        assert!(delta.updated_objects.get(&42).is_some());

        let delta_object = delta.updated_objects.get(&42).unwrap();

        assert_eq!(None, delta_object.shape);
        assert_eq!(Some(object.position), delta_object.position);
        assert_eq!(None, delta_object.mobility);
        assert_eq!(None, delta_object.kind);
        assert_eq!(None, delta_object.sensor);
    }

    #[test]
    fn calculate_deltas_handles_added_object() {
        let object = object_description();

        let first_snapshot = Snapshot::new();

        let mut second_snapshot = Snapshot::new();
        second_snapshot.insert(42, object.clone());

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        assert_eq!(1, delta.updated_objects.len());
        assert_eq!(0, delta.deleted_objects.len());

        assert!(delta.updated_objects.get(&42).is_some());

        let delta_object = delta.updated_objects.get(&42).unwrap();

        assert_eq!(Some(object.shape), delta_object.shape);
        assert_eq!(Some(object.position), delta_object.position);
        assert_eq!(Some(object.mobility), delta_object.mobility);
        assert_eq!(Some(object.kind), delta_object.kind);
        assert_eq!(Some(object.sensor), delta_object.sensor);
    }
}
