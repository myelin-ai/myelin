use crate::controller::{Presenter, Snapshot};
use myelin_environment::object::ObjectDescription;
use myelin_visualization_core::view_model_delta::{
    ObjectDelta, ObjectDescriptionDelta, ViewModelDelta,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct DeltaPresenter;

fn get_object_description_delta(
    first: Option<&ObjectDescription>,
    second: ObjectDescription,
) -> ObjectDescriptionDelta {
    ObjectDescriptionDelta {
        shape: get_delta(first.map(|o| &o.shape), second.shape),
        location: get_delta(
            first.map(|o| &o.position.location),
            second.position.location,
        ),
        rotation: get_delta(
            first.map(|o| &o.position.rotation),
            second.position.rotation,
        ),
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
        let mut deltas: HashMap<_, _> = simulation_snapshot
            .iter()
            .map(|(&id, object)| {
                let delta = if visualized_snapshot.contains_key(&id) {
                    ObjectDelta::Updated(get_object_description_delta(
                        visualized_snapshot.get(&id),
                        object.clone(),
                    ))
                } else {
                    ObjectDelta::Created(object.clone())
                };

                (id, delta)
            })
            .filter(|(_, delta)| match delta {
                ObjectDelta::Created(_) | ObjectDelta::Deleted => true,
                ObjectDelta::Updated(delta) => delta_contains_changes(delta),
            })
            .collect();

        deltas.extend(
            visualized_snapshot
                .keys()
                .filter(|id| !simulation_snapshot.contains_key(id))
                .map(|&id| (id, ObjectDelta::Deleted)),
        );

        deltas
    }
}

fn delta_contains_changes(delta: ObjectDescriptionDelta) -> bool {
    delta.shape.is_some()
        || delta.location.is_some()
        || delta.rotation.is_some()
        || delta.mobility.is_some()
        || delta.kind.is_some()
        || delta.sensor.is_some()
}

impl DeltaPresenter {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
pub(crate) use self::mock::*;

#[cfg(test)]
mod mock {
    use crate::controller::Snapshot;
    use crate::presenter::Presenter;
    use myelin_visualization_core::view_model_delta::ViewModelDelta;
    use std::cell::RefCell;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct PresenterMock {
        expect_calculate_deltas_and_return: Option<(Snapshot, Snapshot, ViewModelDelta)>,
        calculate_deltas_was_called: RefCell<bool>,
    }
    impl PresenterMock {
        pub(crate) fn expect_calculate_deltas(
            &mut self,
            visualized_snapshot: Snapshot,
            simulation_snapshot: Snapshot,
            return_value: ViewModelDelta,
        ) {
            self.expect_calculate_deltas_and_return =
                Some((visualized_snapshot, simulation_snapshot, return_value));
        }
    }
    impl Presenter for PresenterMock {
        fn calculate_deltas(
            &self,
            visualized_snapshot: &Snapshot,
            simulation_snapshot: &Snapshot,
        ) -> ViewModelDelta {
            *self.calculate_deltas_was_called.borrow_mut() = true;

            if let Some((
                ref expected_visualized_snapshot,
                ref expected_simulation_snapshot,
                ref return_value,
            )) = self.expect_calculate_deltas_and_return
            {
                if *visualized_snapshot == *expected_visualized_snapshot
                    && *simulation_snapshot == *expected_simulation_snapshot
                {
                    return_value.clone()
                } else {
                    panic!(
                        "calculate_deltas() was called with {:?} and {:?}, expected {:?} and {:?}",
                        visualized_snapshot,
                        simulation_snapshot,
                        expected_visualized_snapshot,
                        expected_simulation_snapshot,
                    )
                }
            } else {
                panic!("calculate_deltas() was called unexpectedly")
            }
        }
    }

    impl Drop for PresenterMock {
        fn drop(&mut self) {
            if panicking() {
                return;
            }
            if self.expect_calculate_deltas_and_return.is_some() {
                assert!(
                    *self.calculate_deltas_was_called.borrow(),
                    "calculate_deltas() was not called, but expected"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::{Kind, Mobility, ObjectDescription, Radians};
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::collections::HashMap;

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

        object.position.location.x += 10;

        let mut second_snapshot = Snapshot::new();
        second_snapshot.insert(42, object.clone());

        let delta_presenter = DeltaPresenter::default();
        let delta = delta_presenter.calculate_deltas(&first_snapshot, &second_snapshot);

        let expected_delta = ObjectDescriptionDelta {
            shape: None,
            location: Some(object.position.location),
            rotation: None,
            mobility: None,
            kind: None,
            sensor: None,
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

        let expected_object_description = ObjectBuilder::new()
            .shape(object.shape)
            .location(object.position.location.x, object.position.location.y)
            .rotation(object.position.rotation)
            .mobility(object.mobility)
            .kind(object.kind)
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
