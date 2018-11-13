//! Behaviours of various organisms

use myelin_environment::object::*;
use myelin_environment::Id;
use std::collections::HashMap;

/// A purely static and non-interactive behavior.
/// This type will never perform any actions.
#[derive(Debug, Default, Clone)]
pub struct Static;

impl ObjectBehavior for Static {
    fn step(
        &mut self,
        _own_description: &ObjectDescription,
        _sensor_collisions: &HashMap<Id, ObjectDescription>,
    ) -> Option<Action> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_geometry::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn returns_no_actions() {
        let own_description = ObjectBuilder::default()
            .shape(
                PolygonBuilder::default()
                    .vertex(-50.0, -50.0)
                    .vertex(50.0, -50.0)
                    .vertex(50.0, 50.0)
                    .vertex(-50.0, 50.0)
                    .build()
                    .unwrap(),
            )
            .location(300.0, 450.0)
            .rotation(Radians::try_new(FRAC_PI_2).unwrap())
            .kind(Kind::Organism)
            .mobility(Mobility::Movable(Vector { x: 3.0, y: 5.0 }))
            .build()
            .unwrap();
        let mut object = Static::default();
        let action = object.step(&own_description, &HashMap::new());
        assert!(action.is_none());
    }

}
