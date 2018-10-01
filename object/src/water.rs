//! Behaviours of various waters

use myelin_environment::object::*;

/// A purely static and non-interactive water.
/// This type will never perform any actions.
#[derive(Debug, Default, Clone)]
pub struct StaticWater;
impl StaticWater {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for StaticWater {
    fn step(&mut self, _sensor_collisions: &[ObjectDescription]) -> Vec<ImmovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Water
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
        let mut object = StaticWater::new();
        let actions = object.step(&[]);
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticWater::new();
        let kind = object.kind();
        assert_eq!(Kind::Water, kind);
    }

    #[test]
    fn has_no_sensors() {
        let object = StaticWater::new();
        let sensor = object.sensor();
        assert!(sensor.is_none());
    }
}
