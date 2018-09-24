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
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool;
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
    fn flip_coin_with_probability(&mut self, probability: f64) -> bool {
        let num: f64 = self.rng.gen();
        probability <= num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_environment::object::Kind;

    #[derive(Debug)]
    struct RandomChanceCheckerMock {
        coin_flip_result: bool,
    }
    impl RandomChanceCheckerMock {
        fn with_coin_flip_result(coin_flip_result: bool) -> Self {
            Self { coin_flip_result }
        }
    }

    impl RandomChanceChecker for RandomChanceCheckerMock {
        fn flip_coin_with_probability(&mut self, _probability: f64) -> bool {
            self.coin_flip_result
        }
    }

    #[test]
    fn is_correct_kind() {
        let random_chance_checker = Box::new(RandomChanceCheckerMock::with_coin_flip_result(true));
        let object = StochasticSpreadingPlant::new(random_chance_checker);
        let kind = object.kind();
        assert_eq!(Kind::Plant, kind);
    }
}
