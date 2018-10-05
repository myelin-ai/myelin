//! Behaviours of various organisms

use myelin_environment::object::*;

/// A purely static and non-interactive behavior.
/// This type will never perform any actions.
#[derive(Debug, Default)]
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
    ) -> Vec<Action> {
        Vec::new()
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
            .rotation(Radians(FRAC_PI_2))
            .kind(Kind::Organism)
            .mobility(Mobility::Movable(Velocity { x: 3, y: 5 }))
            .build()
            .unwrap();
        let mut object = Static::new();
        let actions = object.step(&own_description, &[]);
        assert!(actions.is_empty());
    }

}
