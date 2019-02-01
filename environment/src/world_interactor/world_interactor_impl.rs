use super::Interactable;
use crate::prelude::*;
use std::time::Duration;

/// Default implementation of [`WorldInteractor`].
///
/// [`WorldInteractor`]: ./../object/trait.WorldInteractor.html
#[derive(Debug)]
pub struct WorldInteractorImpl<'a> {
    interactable: &'a dyn Interactable,
}

impl<'a> WorldInteractorImpl<'a> {
    /// Creates a new instance of [`WorldInteractorImpl`].
    ///
    /// [`WorldInteractorImpl`]: ./struct.WorldInteractorImpl.html
    pub fn new(interactable: &'a dyn Interactable) -> Self {
        Self { interactable }
    }
}

impl<'a> WorldInteractor for WorldInteractorImpl<'a> {
    fn find_objects_in_area(&self, area: Aabb) -> Snapshot {
        self.interactable.objects_in_area(area)
    }

    fn elapsed_time_in_update(&self) -> Duration {
        self.interactable.elapsed_time_in_update()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_interactor::InteractableMock;
    use mockiato::partial_eq;
    use myelin_geometry::{Point, PolygonBuilder};

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .location(10.0, 10.0)
            .mobility(Mobility::Immovable)
            .shape(
                PolygonBuilder::default()
                    .vertex(-5.0, -5.0)
                    .vertex(5.0, 5.0)
                    .vertex(5.0, -5.0)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn find_objects_in_area_is_propagated() {
        let objects = hashmap! { 125 => object_description() };
        let area = Aabb {
            upper_left: Point { x: 10.0, y: 10.0 },
            lower_right: Point { x: 20.0, y: 0.0 },
        };

        let mut interactable = InteractableMock::new();
        interactable
            .expect_objects_in_area(partial_eq(area))
            .returns(objects.clone());
        let object_environment = WorldInteractorImpl::new(&interactable);

        assert_eq!(objects, object_environment.find_objects_in_area(area));
    }
}
