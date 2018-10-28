//! Behaviours of various organisms

use myelin_environment::object::*;

/// A purely static and non-interactive behavior.
/// This type will never perform any actions.
#[derive(Debug, Default, Clone)]
pub struct Static;
impl Static {
    pub fn new() -> Self {
        Self {}
    }
}

impl ObjectBehavior for Static {
    fn step(
        &mut self,
        _own_description: &ObjectDescription,
        _sensor_collisions: &[ObjectDescription],
    ) -> Option<Action> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;
    use myelin_environment::object_builder::{ObjectBuilder, PolygonBuilder};
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn returns_no_actions() {
        let own_description = ObjectBuilder::new()
            .shape(
                PolygonBuilder::new()
                    .vertex(-50, -50)
                    .vertex(50, -50)
                    .vertex(50, 50)
                    .vertex(-50, 50)
                    .build()
                    .unwrap(),
            )
            .location(300, 450)
            .rotation(Radians::try_new(FRAC_PI_2).unwrap())
            .kind(Kind::Organism)
            .mobility(Mobility::Movable(Velocity { x: 3, y: 5 }))
            .build()
            .unwrap();
        let mut object = Static::new();
        let action = object.step(&own_description, &[]);
        assert!(action.is_none());
    }

}
