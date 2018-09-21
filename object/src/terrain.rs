use myelin_environment::object::{ImmovableAction, ImmovableObject, Kind};

#[derive(Debug, Default)]
pub struct StaticTerrain;
impl StaticTerrain {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for StaticTerrain {
    fn step(&mut self) -> Vec<ImmovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Terrain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[test]
    fn has_no_action() {
        let mut object = StaticTerrain::new();
        let actions = object.step();
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticTerrain::new();
        let kind = object.kind();
        assert_eq!(Kind::Terrain, kind);
    }
}
