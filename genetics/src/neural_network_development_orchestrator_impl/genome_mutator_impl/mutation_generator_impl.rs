use crate::genome::Genome;
use crate::neural_network_development_orchestrator_impl::{Mutation, MutationGenerator};
use myelin_random::Random;

pub struct MutationGeneratorImpl {
    random: Box<dyn Random>,
}

impl MutationGeneratorImpl {
    fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }
}

impl MutationGenerator for MutationGeneratorImpl {
    fn generate_mutation(&self, genome: &Genome) -> Mutation {
        unimplemented!()
    }
}

const ADD_NEURON_PROBABILITY: f64 = 5.0 / 100.0;
const ADD_CONNECTION_PROBABILITY: f64 = 7.0 / 100.0;
const DISABLE_CONNECTION_PROBABILITY: f64 = 2.0 / 100.0;
const NUDGE_WEIGHT_PROBABILITY: f64 = 25.0 / 100.0;
const CHANGE_PLACEMENT_NEURON_PROBABILITY: f64 = 5.0 / 1_000.0;
const CHANGE_TARGET_NEURON_PROBABILITY: f64 = 5.0 / 1_000.0;
const ADD_NEW_CLUSTER_PROBABILITY: f64 = 2.0 / 100.0;
const COPY_CLUSTER_PROBABILITY: f64 = 1.0 / 100.0;
const ADD_HOX_WITH_EXISTING_CLUSTER_PROBABILITY: f64 = 1.0 / 100.0;
const DUPLICATE_HOX_PROBABILITY: f64 = 8.0 / 1_000.0;
const BRIDGE_PROBABILITY: f64 = 5.0 / 1_000.0;
const DESYNC_PROBABILITY: f64 = 2.0 / 1_000.0;

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_random::RandomMock;

    #[test]
    fn picks_mutation() {}

    fn mutation_generator() -> Box<dyn MutationGenerator> {
        box MutationGeneratorImpl::new(mock_random())
    }

    fn mock_random() -> Box<dyn Random> {
        let mut random = box RandomMock::new();
        random
            .expect_flip_coin_with_probability(|arg| arg.any())
            .times(..)
            .returns(false);
        random
    }
}
