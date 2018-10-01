//! Behaviours of various plants

use myelin_environment::object::*;

/// A purely static and non-interactive plant.
/// This type will never perform any actions.
#[derive(Debug, Default)]
pub struct StaticPlant;
impl StaticPlant {
    pub fn new() -> Self {
        Self {}
    }
}

impl MovableObject for StaticPlant {
    fn step(&mut self, _sensor_collisions: &[ObjectDescription]) -> Vec<MovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Plant
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
        let mut object = StaticPlant::new();
        let actions = object.step(&[]);
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticPlant::new();
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }

    #[test]
    fn has_no_sensors() {
        let object = StaticPlant::new();
        let sensor = object.sensor();
        assert!(sensor.is_none());
    }
}
