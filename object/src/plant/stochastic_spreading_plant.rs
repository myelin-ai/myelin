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

    #[derive(Debug)]
    struct RandomChanceCheckerMock {
        did_chance_occur_return_value: bool,
    }
    impl RandomChanceCheckerMock {
        fn with_return_value(return_value: bool) -> Self {
            Self {
                did_chance_occur_return_value: return_value,
            }
        }
    }

    impl RandomChanceChecker for RandomChanceCheckerMock {
        fn did_chance_occur(&self, _chance: f32) -> bool {
            self.did_chance_occur_return_value
        }
    }

    #[test]
    fn is_correct_kind() {
        let random_chance_checker = Box::new(RandomChanceCheckerMock::with_return_value(true));
        let object = StochasticSpreadingPlant::new(random_chance_checker);
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }
}
