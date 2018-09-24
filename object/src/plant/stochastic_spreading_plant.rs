use myelin_environment::object::{ImmovableAction, ImmovableObject, Kind};
use std::fmt;

/// Plant that spreads in stochastic intervals
#[derive(Debug)]
pub struct StochasticSpreadingPlant {
    random_chance_checker: Box<dyn RandomChanceChecker>,
}

impl StochasticSpreadingPlant {
    pub fn new(random_chance_checker: Box<dyn RandomChanceChecker>) -> Self {
        Self {
            random_chance_checker,
        }
    }
}

impl ImmovableObject for StochasticSpreadingPlant {
    fn step(&mut self) -> Vec<ImmovableAction> {
        unimplemented!()
    }
    fn kind(&self) -> Kind {
        Kind::Plant
    }
}

pub trait RandomChanceChecker: fmt::Debug {
    fn did_chance_occur(&self, chance: f32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[test]
    fn is_correct_kind() {
        let object = StochasticSpreadingPlant::new();
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }
}
