use super::{CorpusCallosumClusterGeneGenerator, CorpusCallosumConfiguration};
use crate::genome::*;
use myelin_random::Random;

/// Default implementation of [`IoClusterGeneGenerator`]
#[derive(Debug)]
pub struct CorpusCallosumClusterGeneGeneratorImpl {
    random: Box<dyn Random>,
}

impl CorpusCallosumClusterGeneGeneratorImpl {
    /// Creates a new [`CorbusCallossumClusterGeneGeneratorImpl`].
    pub fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }
}

impl CorpusCallosumClusterGeneGenerator for CorpusCallosumClusterGeneGeneratorImpl {
    fn generate_cluster_gene(&self, configuration: &CorpusCallosumConfiguration) -> ClusterGene {
        let neuron_count =
            configuration.input_neuron_count.get() + configuration.output_neuron_count.get();
        ClusterGene {
            neurons: vec![Neuron {}; neuron_count],
            connections: Vec::new(),
            specialization: CLUSTER_GENE_SPECIALIZATION,
            placement_neuron: PLACEMENT_NEURON,
        }
    }
}

/// Chosen arbitrarily
const MIN_CONNECTION_WEIGHT: f64 = 0.000_000_1;
const MAX_CONNECTION_WEIGHT: f64 = 1.0;
const CLUSTER_GENE_SPECIALIZATION: ClusterGeneSpecialization = ClusterGeneSpecialization::Initial;
/// Not relevant since the initial cluster is placed standalone
const PLACEMENT_NEURON: NeuronClusterLocalIndex = NeuronClusterLocalIndex('✌' as usize);

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_random::RandomMock;
    use std::num::NonZeroUsize;

    #[test]
    fn generates_correct_amount_of_neurons_and_connections() {
        const INPUT_NEURON_COUNT: usize = 5;
        const OUTPUT_NEURON_COUNT: usize = 2;
        let random = {
            let mut random = RandomMock::new();
            random
                .expect_random_float_in_range(
                    partial_eq(MIN_CONNECTION_WEIGHT),
                    partial_eq(MAX_CONNECTION_WEIGHT),
                )
                .times(..)
                .returns(1.0);
            box random
        };
        let generator = CorpusCallosumClusterGeneGeneratorImpl::new(random);
        let configuration = configuration(INPUT_NEURON_COUNT, OUTPUT_NEURON_COUNT);
        let expected_neurons = vec![Neuron {}; INPUT_NEURON_COUNT + OUTPUT_NEURON_COUNT];
        let generated_cluster_gene = generator.generate_cluster_gene(&configuration);
        assert_eq!(expected_neurons, generated_cluster_gene.neurons,);
    }

    fn configuration(
        input_neuron_count: usize,
        output_neuron_count: usize,
    ) -> CorpusCallosumConfiguration {
        CorpusCallosumConfiguration {
            input_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
            output_neuron_count: NonZeroUsize::new(output_neuron_count).unwrap(),
        }
    }
}
