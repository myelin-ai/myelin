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

const ADD_NEURON_PROBABILITY: f64 = 3.0 / 100.0;

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
            .times(..);
        random
    }
}
