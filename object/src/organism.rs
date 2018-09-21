use myelin_environment::object::*;

#[derive(Debug, Default)]
pub struct StaticOrganism;
impl StaticOrganism {
    fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for StaticOrganism {
    fn step(&mut self) -> Vec<ImmovableAction> {
        Vec::new()
    }
    fn kind(&self) -> Kind {
        Kind::Organism
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
}
