use crate::genome::Genome;
use crate::orchestrator_impl::{
    NeuralNetworkBuilder, NeuralNetworkDeveloper, NeuralNetworkFactory,
};
use crate::NeuralNetworkDevelopmentConfiguration;
use nameof::name_of;
use std::fmt::{self, Debug};
use std::rc::Rc;

/// Bootstraps the neural network based on the genome
#[derive(Debug, Clone)]
pub struct GeneticNeuralNetworkDeveloper {
    development_configuration: NeuralNetworkDevelopmentConfiguration,
    genome: Genome,
}

impl GeneticNeuralNetworkDeveloper {
    pub fn new(
        development_configuration: NeuralNetworkDevelopmentConfiguration,
        genome: Genome,
    ) -> Self {
        Self {
            development_configuration,
            genome,
        }
    }
}

impl NeuralNetworkDeveloper for GeneticNeuralNetworkDeveloper {
    fn develop_neural_network(self: Box<Self>, builder: &mut NeuralNetworkBuilder) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::{
        ClusterGene, ClusterGeneIndex, Connection as ConnectionDefinition, HoxGene, HoxPlacement,
        Neuron as NeuronDefinition, NeuronClusterLocalIndex,
    };

    #[test]
    fn creates_standalone_cluster_properly() {
        let mut genome = test_genome_stub();
        genome = add_cluster_zero_to_genome(genome);
        genome = add_hox_gene_zero_to_genome(genome);
        let mut config = test_config();

        let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
        let mut writer = NeuralNetworkBuilder {};

        developer.develop_neural_network(&mut writer);

        // TODO: actual check if the developer did everything right
        unimplemented!("unfinished");
    }

    fn test_config() -> NeuralNetworkDevelopmentConfiguration {
        NeuralNetworkDevelopmentConfiguration {
            parent_genomes: (Genome::default(), Genome::default()),
            input_neuron_count: 1,
            output_neuron_count: 1,
        }
    }

    fn test_genome_stub() -> Genome {
        Genome::default()
    }

    fn add_cluster_zero_to_genome(mut genome: Genome) -> Genome {
        genome.cluster_genes.insert(
            0,
            ClusterGene {
                neurons: vec![
                    NeuronDefinition {},
                    NeuronDefinition {},
                    NeuronDefinition {},
                    NeuronDefinition {},
                ],
                connections: vec![
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(0),
                        to: NeuronClusterLocalIndex(1),
                        weight: 0.5,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(0),
                        to: NeuronClusterLocalIndex(2),
                        weight: 0.7,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(0),
                        to: NeuronClusterLocalIndex(3),
                        weight: 0.2,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(3),
                        to: NeuronClusterLocalIndex(1),
                        weight: 0.3,
                    },
                ],
                placement_neuron: NeuronClusterLocalIndex(1),
            },
        );

        genome
    }

    fn add_cluster_one_to_genome(mut genome: Genome) -> Genome {
        genome.cluster_genes.insert(
            1,
            ClusterGene {
                neurons: vec![
                    NeuronDefinition {},
                    NeuronDefinition {},
                    NeuronDefinition {},
                ],
                connections: vec![
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(0),
                        to: NeuronClusterLocalIndex(2),
                        weight: 0.4,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(1),
                        to: NeuronClusterLocalIndex(2),
                        weight: 0.6,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(2),
                        to: NeuronClusterLocalIndex(0),
                        weight: 0.45,
                    },
                    ConnectionDefinition {
                        from: NeuronClusterLocalIndex(2),
                        to: NeuronClusterLocalIndex(1),
                        weight: 0.82,
                    },
                ],
                placement_neuron: NeuronClusterLocalIndex(0),
            },
        );

        genome
    }

    fn add_hox_gene_zero_to_genome(mut genome: Genome) -> Genome {
        genome.hox_genes.insert(
            0,
            HoxGene {
                placement: HoxPlacement::Standalone,
                cluster_index: ClusterGeneIndex(0),
                disabled_connections: vec![],
            },
        );

        genome
    }

}
