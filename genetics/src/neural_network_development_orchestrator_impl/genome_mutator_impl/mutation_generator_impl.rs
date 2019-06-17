use crate::genome::Genome;
use crate::neural_network_development_orchestrator_impl::{Mutation, MutationGenerator};
use myelin_random::Random;

pub struct MutationGeneratorImpl {
    random: Box<dyn Random>,
}

impl MutationGeneratorImpl {
    fn new(random: Box<dyn Random>) -> Self {
        random
    }
}

impl MutationGenerator for MutationGeneratorImpl {
    fn generate_mutation(&self, genome: &Genome) -> Mutation {
        unimplemented!()
    }
}
