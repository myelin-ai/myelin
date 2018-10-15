use crate::controller::Presenter;
use myelin_environment::object::ObjectDescription;
use myelin_environment::Id;
use myelin_visualization_core::view_model_delta::{ObjectDescriptionDelta, ViewModelDelta};
use std::collections::HashMap;

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
        visualized_objects: &HashMap<Id, ObjectDescription>,
        simulated_objects: &HashMap<Id, ObjectDescription>,
    ) -> ViewModelDelta {
        let deleted_objects = visualized_objects
            .keys()
            .filter(|id| !simulated_objects.contains_key(id))
            .map(|&id| id)
            .collect();

        let updated_objects = simulated_objects
            .iter()
            .map(|(&id, object_description)| {
                (
                    id,
                    get_object_description_delta(
                        visualized_objects.get(&id),
                        object_description.clone(),
                    ),
                )
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
    use std::cell::RefCell;
    use std::error::Error;
    use std::f64::consts::PI;
    use std::thread;

    fn object_description(orientation: Radians) -> ObjectDescription {
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
            .rotation(orientation)
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

}
