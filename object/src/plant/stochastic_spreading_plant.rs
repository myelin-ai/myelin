use myelin_environment::object::{ImmovableAction, ImmovableObject, Kind};
use rand::{thread_rng, Rng, ThreadRng};
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
    fn did_chance_occur(&mut self, chance: f64) -> bool;
}

#[derive(Debug)]
pub struct RandomChanceCheckerImpl {
    rng: ThreadRng,
}

impl RandomChanceCheckerImpl {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl RandomChanceChecker for RandomChanceCheckerImpl {
    fn did_chance_occur(&mut self, chance: f64) -> bool {
        let num: f64 = self.rng.gen();
        chance <= num
    }
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
        fn did_chance_occur(&mut self, _chance: f64) -> bool {
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
