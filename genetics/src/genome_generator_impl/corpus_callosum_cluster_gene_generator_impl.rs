use super::{CorpusCallosumClusterGeneGenerator, CorpusCallosumConfiguration};
use crate::genome::*;
use myelin_random::Random;
use std::iter;

/// Default implementation of [`IoClusterGeneGenerator`]
#[derive(Debug)]
pub struct CorpusCallosumClusterGeneGeneratorImpl {
    random: Box<dyn Random>,
}

impl CorpusCallosumClusterGeneGeneratorImpl {
    /// Creates a new [`CorpusCallossumClusterGeneGeneratorImpl`].
    pub fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }
}

impl CorpusCallosumClusterGeneGenerator for CorpusCallosumClusterGeneGeneratorImpl {
    fn generate_cluster_gene(&self, configuration: &CorpusCallosumConfiguration) -> ClusterGene {
        let output_neuron_count = configuration.output_neuron_count.get();
        let input_neuron_count = configuration.input_neuron_count.get();
        let stem_neuron_count = self
            .random
            .random_usize_in_range(MIN_STEM_NEURONS, output_neuron_count);
        let neuron_count = input_neuron_count + output_neuron_count + stem_neuron_count;
        let connections = self
            .generate_input_neuron_connections(input_neuron_count)
            .collect();
        ClusterGene {
            neurons: vec![Neuron {}; neuron_count],
            connections,
            specialization: CLUSTER_GENE_SPECIALIZATION,
            placement_neuron: PLACEMENT_NEURON,
        }
    }
}

impl CorpusCallosumClusterGeneGeneratorImpl {
    fn generate_input_neuron_connections<'a>(
        &'a self,
        input_neuron_count: usize,
    ) -> impl Iterator<Item = Connection> + 'a {
        (0..input_neuron_count)
            .zip((0..input_neuron_count).cycle().skip(1))
            .take(input_neuron_count)
            .map(move |(from_index, to_index)| {
                let connection = self.generate_input_neuron_connection(from_index, to_index);
                let connection_reverse =
                    self.generate_input_neuron_connection(to_index, from_index);

                iter::once(connection).chain(iter::once(connection_reverse))
            })
            .flatten()
    }

    fn generate_input_neuron_connection(&self, from_index: usize, to_index: usize) -> Connection {
        let weight = self
            .random
            .random_float_in_range(MIN_CONNECTION_WEIGHT, MAX_CONNECTION_WEIGHT);
        Connection {
            from: NeuronClusterLocalIndex(from_index),
            to: NeuronClusterLocalIndex(to_index),
            weight,
        }
    }
}

const MIN_STEM_NEURONS: usize = 1;
/// Chosen arbitrarily
const MIN_CONNECTION_WEIGHT: f64 = 0.000_000_1;
const MAX_CONNECTION_WEIGHT: f64 = 1.0;
const CLUSTER_GENE_SPECIALIZATION: ClusterGeneSpecialization = ClusterGeneSpecialization::Initial;
/// Not relevant since the initial cluster is placed standalone
const PLACEMENT_NEURON: NeuronClusterLocalIndex = NeuronClusterLocalIndex('✌' as usize);

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::{any, partial_eq};
    use myelin_random::RandomMock;
    use std::num::NonZeroUsize;

    #[test]
    fn generates_correct_amount_of_neurons_and_connections() {
        const INPUT_NEURON_COUNT: usize = 5;
        const OUTPUT_NEURON_COUNT: usize = 2;
        const STEM_NEURONS: usize = 5;
        const NEURONS: usize = INPUT_NEURON_COUNT + OUTPUT_NEURON_COUNT + STEM_NEURONS;
        let random = {
            let mut random = RandomMock::new();
            random
                .expect_random_float_in_range(any(), any())
                .times(..)
                .returns(1.0);
            random
                .expect_random_usize_in_range(
                    partial_eq(MIN_STEM_NEURONS),
                    partial_eq(OUTPUT_NEURON_COUNT),
                )
                .returns(STEM_NEURONS);
            box random
        };
        let generator = CorpusCallosumClusterGeneGeneratorImpl::new(random);
        let configuration = configuration(INPUT_NEURON_COUNT, OUTPUT_NEURON_COUNT);
        let expected_neurons = vec![Neuron {}; NEURONS];
        let generated_cluster_gene = generator.generate_cluster_gene(&configuration);
        assert_eq!(expected_neurons, generated_cluster_gene.neurons);
    }

    #[test]
    fn input_neurons_are_connected_in_circle() {
        const INPUT_NEURON_COUNT: usize = 5;
        const OUTPUT_NEURON_COUNT: usize = 2;
        const STEM_NEURONS: usize = 5;
        const NEURONS: usize = INPUT_NEURON_COUNT + OUTPUT_NEURON_COUNT + STEM_NEURONS;
        const INPUT_NEURON_CONNECTIONS: usize = INPUT_NEURON_COUNT * 2;

        let random = {
            let mut random = RandomMock::new();
            random.expect_random_float_in_range_calls_in_order();
            for index in 0..INPUT_NEURON_CONNECTIONS {
                random
                    .expect_random_float_in_range(
                        partial_eq(MIN_CONNECTION_WEIGHT),
                        partial_eq(MAX_CONNECTION_WEIGHT),
                    )
                    .returns(connection_weight(index));
            }
            random
                .expect_random_usize_in_range(
                    partial_eq(MIN_STEM_NEURONS),
                    partial_eq(OUTPUT_NEURON_COUNT),
                )
                .returns(STEM_NEURONS);
            box random
        };
        let generator = CorpusCallosumClusterGeneGeneratorImpl::new(random);
        let configuration = configuration(INPUT_NEURON_COUNT, OUTPUT_NEURON_COUNT);
        let expected_neurons = vec![Neuron {}; NEURONS];
        let expected_input_connections = vec![
            Connection {
                from: NeuronClusterLocalIndex(0),
                to: NeuronClusterLocalIndex(1),
                weight: connection_weight(0),
            },
            Connection {
                from: NeuronClusterLocalIndex(1),
                to: NeuronClusterLocalIndex(0),
                weight: connection_weight(1),
            },
            Connection {
                from: NeuronClusterLocalIndex(1),
                to: NeuronClusterLocalIndex(2),
                weight: connection_weight(2),
            },
            Connection {
                from: NeuronClusterLocalIndex(2),
                to: NeuronClusterLocalIndex(1),
                weight: connection_weight(3),
            },
            Connection {
                from: NeuronClusterLocalIndex(2),
                to: NeuronClusterLocalIndex(3),
                weight: connection_weight(4),
            },
            Connection {
                from: NeuronClusterLocalIndex(3),
                to: NeuronClusterLocalIndex(2),
                weight: connection_weight(5),
            },
            Connection {
                from: NeuronClusterLocalIndex(3),
                to: NeuronClusterLocalIndex(4),
                weight: connection_weight(6),
            },
            Connection {
                from: NeuronClusterLocalIndex(4),
                to: NeuronClusterLocalIndex(3),
                weight: connection_weight(7),
            },
            Connection {
                from: NeuronClusterLocalIndex(4),
                to: NeuronClusterLocalIndex(0),
                weight: connection_weight(8),
            },
            Connection {
                from: NeuronClusterLocalIndex(0),
                to: NeuronClusterLocalIndex(4),
                weight: connection_weight(9),
            },
        ];
        let generated_cluster_gene = generator.generate_cluster_gene(&configuration);
        assert_eq!(expected_neurons, generated_cluster_gene.neurons);
        assert_eq!(
            expected_input_connections,
            &generated_cluster_gene.connections[0..expected_input_connections.len()]
        );
    }

    #[test]
    fn input_neurons_are_correctly_connected_with_stem_neurons() {
        const INPUT_NEURON_COUNT: usize = 5;
        const OUTPUT_NEURON_COUNT: usize = 2;
        const STEM_NEURONS: usize = 3;
        const NEURONS: usize = INPUT_NEURON_COUNT + OUTPUT_NEURON_COUNT + STEM_NEURONS;
        const INPUT_NEURON_CONNECTIONS: usize = INPUT_NEURON_COUNT * 2;
        const INPUT_TO_STEM_CONNECTIONS: usize = INPUT_NEURON_COUNT;

        let random = {
            let mut random = RandomMock::new();
            random.expect_random_float_in_range_calls_in_order();
            random
                .expect_random_float_in_range(any(), any())
                .times(INPUT_NEURON_CONNECTIONS as u64)
                .returns(1.0);
            for index in 0..INPUT_TO_STEM_CONNECTIONS {
                random
                    .expect_random_float_in_range(
                        partial_eq(MIN_CONNECTION_WEIGHT),
                        partial_eq(MAX_CONNECTION_WEIGHT),
                    )
                    .returns(connection_weight(index));
            }
            random
                .expect_random_usize_in_range(
                    partial_eq(MIN_STEM_NEURONS),
                    partial_eq(OUTPUT_NEURON_COUNT),
                )
                .returns(STEM_NEURONS);
            box random
        };
        let generator = CorpusCallosumClusterGeneGeneratorImpl::new(random);
        let configuration = configuration(INPUT_NEURON_COUNT, OUTPUT_NEURON_COUNT);
        let expected_neurons = vec![Neuron {}; NEURONS];
        let expected_input_to_stem_connections = vec![
            Connection {
                from: NeuronClusterLocalIndex(0),
                to: NeuronClusterLocalIndex(INPUT_NEURON_COUNT),
                weight: connection_weight(0),
            },
            Connection {
                from: NeuronClusterLocalIndex(1),
                to: NeuronClusterLocalIndex(INPUT_NEURON_COUNT),
                weight: connection_weight(0),
            },
            Connection {
                from: NeuronClusterLocalIndex(2),
                to: NeuronClusterLocalIndex(INPUT_NEURON_COUNT + 1),
                weight: connection_weight(0),
            },
            Connection {
                from: NeuronClusterLocalIndex(3),
                to: NeuronClusterLocalIndex(INPUT_NEURON_COUNT + 1),
                weight: connection_weight(0),
            },
            Connection {
                from: NeuronClusterLocalIndex(4),
                to: NeuronClusterLocalIndex(INPUT_NEURON_COUNT + 2),
                weight: connection_weight(0),
            },
        ];
        let generated_cluster_gene = generator.generate_cluster_gene(&configuration);
        let max_stem_connections_index =
            INPUT_NEURON_CONNECTIONS + expected_input_to_stem_connections.len();
        assert_eq!(expected_neurons, generated_cluster_gene.neurons);
        assert_eq!(
            expected_input_to_stem_connections,
            &generated_cluster_gene.connections
                [INPUT_NEURON_CONNECTIONS..(max_stem_connections_index)]
        );
    }

    fn connection_weight(index: usize) -> f64 {
        index as f64
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
