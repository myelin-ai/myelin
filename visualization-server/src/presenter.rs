use crate::controller::Presenter;
use crate::snapshot::SnapshotSlice;
use myelin_visualization_core::view_model_delta::ViewModelDelta;

#[derive(Debug, Default)]
pub(crate) struct DeltaPresenter;

impl Presenter for DeltaPresenter {
    fn calculate_deltas(
        &self,
        last_objects: &SnapshotSlice,
        current_objects: &SnapshotSlice,
    ) -> ViewModelDelta {
        unimplemented!()
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

}
