use myelin_environment::object::*;

#[derive(Debug, Default)]
pub struct Plant;
impl Plant {
    fn new() -> Self {
        Self {}
    }
}

impl ImmovableObject for Plant {
    fn step(&mut self) -> Vec<ImmovableAction> {
        unimplemented!()
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
    fn is_correct_kind() {
        let object = Plant::new();
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }
}
