//! Behaviours of various organisms

use myelin_environment::object::{Kind, MovableAction, MovableObject, Sensor};

/// A purely static and non-interactive organism.
/// This type will never perform any actions.
#[derive(Debug, Default)]
pub struct StaticOrganism;
impl StaticOrganism {
    pub fn new() -> Self {
        Self {}
    }
}

impl MovableObject for StaticOrganism {
    fn step(&mut self) -> Vec<MovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Organism
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
        let mut object = StaticOrganism::new();
        let actions = object.step();
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticOrganism::new();
        let kind = object.kind();
        assert_eq!(Kind::Organism, kind);
    }

    #[test]
    fn has_no_sensors() {
        let object = StaticOrganism::new();
        let sensor = object.sensor();
        assert!(sensor.is_none());
    }
}
