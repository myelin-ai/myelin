use myelin_environment::object::*;

#[derive(Debug, Default)]
pub struct StaticPlant;
impl StaticPlant {
    fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for StaticPlant {
    fn step(&mut self) -> Vec<ImmovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Plant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[test]
    fn has_no_action() {
        let mut object = StaticPlant::new();
        let actions = object.step();
        assert!(actions.is_empty());
    }

    #[test]
    fn is_correct_kind() {
        let object = StaticPlant::new();
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }
}
