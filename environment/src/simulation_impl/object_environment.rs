use crate::object::ObjectEnvironment;
use crate::{Simulation, Snapshot};
use myelin_geometry::Aabb;

/// Default implementation of [`ObjectEnvironment`].
///
/// [`ObjectEnvironment`]: ./../object/trait.ObjectEnvironment.html
#[derive(Debug)]
pub struct ObjectEnvironmentImpl<'a> {
    simulation: &'a dyn Simulation,
}

impl<'a> ObjectEnvironmentImpl<'a> {
    /// Creates a new instance of [`ObjectEnvironmentImpl`].
    ///
    /// [`ObjectEnvironmentImpl`]: ./struct.ObjectEnvironmentImpl.html
    pub fn new(simulation: &'a dyn Simulation) -> Self {
        Self { simulation }
    }
}

impl<'a> ObjectEnvironment for ObjectEnvironmentImpl<'a> {
    fn find_objects_in_area(&self, area: Aabb) -> Snapshot {
        self.simulation.objects_in_area(area)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::object::*;
    use crate::object_builder::ObjectBuilder;
    use crate::SimulationMock;
    use myelin_geometry::{Point, PolygonBuilder};

    fn object_description() -> ObjectDescription {
        ObjectBuilder::default()
            .kind(Kind::Organism)
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

        let mut simulation = SimulationMock::default();
        simulation.expect_objects_in_area_and_return(area, objects.clone());
        let object_environment = ObjectEnvironmentImpl::new(&simulation);

        assert_eq!(objects, object_environment.find_objects_in_area(area));
    }
}
