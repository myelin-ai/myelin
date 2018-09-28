//! Behaviours of various terrains

use myelin_environment::object::*;

/// A purely static and non-interactive terrain.
/// This type will never perform any actions.
#[derive(Debug, Default)]
pub struct StaticTerrain;
impl StaticTerrain {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for StaticTerrain {
    fn step(&mut self, _sensor_collisions: &[ObjectDescription]) -> Vec<ImmovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Terrain
    }
    fn sensor(&self) -> Option<Sensor> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[test]
    fn has_no_action() {
        let mut object = StaticTerrain::new();
        let actions = object.step(&[]);
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticTerrain::new();
        let kind = object.kind();
        assert_eq!(Kind::Terrain, kind);
    }

    #[test]
    fn has_no_sensors() {
        let object = StaticTerrain::new();
        let sensor = object.sensor();
        assert!(sensor.is_none());
    }
}
