use crate::genome::Genome;
use crate::neural_network_development_orchestrator_impl::{Mutation, MutationGenerator};
use myelin_random::{Random, Roulette, RouletteFactory};

pub struct MutationGeneratorImpl {
    random: Box<dyn Random>,
    roulette: Box<dyn Roulette<MutationMarker>>,
}

impl MutationGeneratorImpl {
    fn new(
        random: Box<dyn Random>,
        roulette_factory: Box<dyn RouletteFactory<'static, MutationMarker>>,
    ) -> Self {
        Self {
            random,
            roulette: roulette_factory(Vec::new()),
        }
    }
}

impl MutationGenerator for MutationGeneratorImpl {
    fn generate_mutation(&self, genome: &Genome) -> Mutation {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
enum MutationMarker {
    AddNeuron,
    AddConnection,
    DisableConnection,
    NudgeWeight,
    ChangePlacementNeuron,
    AddNewCluster,
    CopyCluster,
    DesyncCluster,
    Bridge,
    AddHoxWithExistingCluster,
    ChangeTargetNeuron,
    DuplicateHox,
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
    use crate::genome::{
        ClusterConnectionIndex, ClusterGene, ClusterGeneIndex, ClusterNeuronIndex, Connection,
        Neuron,
    };
    use myelin_random::{RandomMock, RouletteMock};

    #[test]
    fn picks_mutation() {
        let genome = genome();
        let mut random = mock_random();
        random
            .expect_flip_coin_with_probability(|arg| arg.partial_eq(ADD_NEURON_PROBABILITY))
            .times(..)
            .returns(true);
        random
            .expect_usize_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(2))
            .returns(1);
        random
            .expect_usize_in_range(|arg| arg.partial_eq(0), |arg| arg.partial_eq(3))
            .returns(1);
        random
            .expect_f64_in_range(|arg| arg.partial_eq(-1.0), |arg| arg.partial_eq(1.0))
            .returns(0.5);

        let mutation_generator = box MutationGeneratorImpl::new(box random, box |_| {
            let mut roulette = RouletteMock::new();
            roulette.expect_spin().returns(MutationMarker::AddNeuron);
            box roulette
        });

        let mutation = mutation_generator.generate_mutation(&genome);
        let expected_mutation = Mutation::AddNeuron {
            cluster_gene: ClusterGeneIndex(1),
            connection: ClusterConnectionIndex(1),
            new_connection_weight: 0.5,
        };
    }

    fn mock_random<'a>() -> RandomMock<'a> {
        let mut random = RandomMock::new();
        random
            .expect_flip_coin_with_probability(|arg| arg.any())
            .times(..)
            .returns(false);
        random
    }

    fn genome() -> Genome {
        Genome {
            cluster_genes: vec![interconnected_cluster_gene(); 2],
            hox_genes: vec![],
        }
    }

    fn interconnected_cluster_gene() -> ClusterGene {
        ClusterGene {
            neurons: vec![Neuron {}; 4],
            connections: vec![
                Connection {
                    from: ClusterNeuronIndex(0),
                    to: ClusterNeuronIndex(1),
                    weight: 1.0,
                },
                Connection {
                    from: ClusterNeuronIndex(1),
                    to: ClusterNeuronIndex(2),
                    weight: 1.0,
                },
                Connection {
                    from: ClusterNeuronIndex(2),
                    to: ClusterNeuronIndex(3),
                    weight: 1.0,
                },
            ],
            placement_neuron: ClusterNeuronIndex(0),
            specialization: Default::default(),
        }
    }
}
