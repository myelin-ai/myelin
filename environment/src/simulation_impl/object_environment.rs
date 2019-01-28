use crate::object::WorldInteractor;
use crate::{Simulation, Snapshot};
use myelin_geometry::Aabb;

/// Default implementation of [`WorldInteractor`].
///
/// [`WorldInteractor`]: ./../object/trait.WorldInteractor.html
#[derive(Debug)]
pub struct WorldInteractorImpl<'a> {
    simulation: &'a dyn Simulation,
}

impl<'a> WorldInteractorImpl<'a> {
    /// Creates a new instance of [`WorldInteractorImpl`].
    ///
    /// [`WorldInteractorImpl`]: ./struct.WorldInteractorImpl.html
    pub fn new(simulation: &'a dyn Simulation) -> Self {
        Self { simulation }
    }
}

impl<'a> WorldInteractor for WorldInteractorImpl<'a> {
    fn find_objects_in_area(&self, area: Aabb) -> Snapshot {
        self.simulation.objects_in_area(area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::*;
    use crate::object_builder::ObjectBuilder;
    use crate::SimulationMock;
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

        let mut simulation = SimulationMock::new();
        simulation
            .expect_objects_in_area(partial_eq(area))
            .returns(objects.clone());
        let object_environment = WorldInteractorImpl::new(&simulation);

        assert_eq!(objects, object_environment.find_objects_in_area(area));
    }
}
