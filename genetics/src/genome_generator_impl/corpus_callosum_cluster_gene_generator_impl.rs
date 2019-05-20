use super::{CorpusCallosumClusterGeneGenerator, CorpusCallosumConfiguration};
use crate::genome::*;
use myelin_associate_lists::associate_lists;
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
        // Using the output neuron count is mostly an arbitrary choice.
        // However `output_neuron_count` is usually smaller than `input_neuron_count` which speaks in its favour.
        let stem_neuron_count = self
            .random
            .random_usize_in_range(MIN_STEM_NEURONS, output_neuron_count + 1);
        let neuron_count = input_neuron_count + output_neuron_count + stem_neuron_count;
        let connections =
            self.generate_input_neuron_connections(input_neuron_count)
                .chain(self.generate_input_to_stem_neuron_connections(
                    input_neuron_count,
                    stem_neuron_count,
                ))
                .chain(self.generate_stem_to_output_neuron_connections(
                    input_neuron_count,
                    stem_neuron_count,
                    output_neuron_count,
                ))
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
                let connection = self.generate_connection(from_index, to_index);
                let connection_reverse = self.generate_connection(to_index, from_index);

                iter::once(connection).chain(iter::once(connection_reverse))
            })
            .flatten()
    }

    fn generate_input_to_stem_neuron_connections<'a>(
        &'a self,
        input_neuron_count: usize,
        stem_neuron_count: usize,
    ) -> impl Iterator<Item = Connection> + 'a {
        let input_neuron_indexes: Vec<_> = (0..input_neuron_count).collect();
        let stem_neuron_indices: Vec<_> =
            (input_neuron_count..(input_neuron_count + stem_neuron_count)).collect();

        associate_lists(&input_neuron_indexes, &stem_neuron_indices)
            .into_iter()
            .map(move |(from_index, to_index)| self.generate_connection(from_index, to_index))
    }

    fn generate_stem_to_output_neuron_connections<'a>(
        &'a self,
        input_neuron_count: usize,
        stem_neuron_count: usize,
        output_neuron_count: usize,
    ) -> impl Iterator<Item = Connection> + 'a {
        let output_neuron_start_index = input_neuron_count + stem_neuron_count;
        let output_neuron_indices: Vec<_> = (output_neuron_start_index
            ..(output_neuron_start_index + output_neuron_count))
            .collect();
        let stem_neuron_indices: Vec<_> =
            (input_neuron_count..(input_neuron_count + stem_neuron_count)).collect();

        associate_lists(&stem_neuron_indices, &output_neuron_indices)
            .into_iter()
            .map(move |(from_index, to_index)| self.generate_connection(from_index, to_index))
    }

    fn generate_connection(&self, from_index: usize, to_index: usize) -> Connection {
        let weight = self
            .random
            .random_float_in_range(MIN_CONNECTION_WEIGHT, MAX_CONNECTION_WEIGHT);
        Connection {
            from: ClusterNeuronIndex(from_index),
            to: ClusterNeuronIndex(to_index),
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
const PLACEMENT_NEURON: ClusterNeuronIndex = ClusterNeuronIndex('✌' as usize);

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_random::RandomMock;
    use std::num::NonZeroUsize;

    #[test]
    fn generates_correct_neurons_and_connections() {
        const INPUT_NEURON_COUNT: usize = 5;
        const STEM_NEURON_COUNT: usize = 3;
        const OUTPUT_NEURON_COUNT: usize = 2;

        let expected_input_connections = vec![
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
            (2, 3),
            (3, 2),
            (3, 4),
            (4, 3),
            (4, 0),
            (0, 4),
        ];

        let expected_input_to_stem_connections = vec![
            (0, INPUT_NEURON_COUNT),
            (1, INPUT_NEURON_COUNT + 1),
            (2, INPUT_NEURON_COUNT + 1),
            (3, INPUT_NEURON_COUNT + 2),
            (4, INPUT_NEURON_COUNT + 2),
        ];

        const FIRST_OUTPUT_NEURON_INDEX: usize = INPUT_NEURON_COUNT + STEM_NEURON_COUNT;
        let expected_stem_to_output_connections = vec![
            (INPUT_NEURON_COUNT, FIRST_OUTPUT_NEURON_INDEX),
            (INPUT_NEURON_COUNT + 1, FIRST_OUTPUT_NEURON_INDEX + 1),
            (INPUT_NEURON_COUNT + 2, FIRST_OUTPUT_NEURON_INDEX + 1),
        ];

        test_connections_and_neurons_are_generated_correctly(ConnectionsTestConfiguration {
            input_neuron_count: INPUT_NEURON_COUNT,
            stem_neuron_count: STEM_NEURON_COUNT,
            output_neuron_count: OUTPUT_NEURON_COUNT,
            expected_connections: expected_input_connections
                .into_iter()
                .chain(expected_input_to_stem_connections)
                .chain(expected_stem_to_output_connections)
                .enumerate()
                .map(|(index, (from, to))| Connection {
                    from: ClusterNeuronIndex(from),
                    to: ClusterNeuronIndex(to),
                    weight: connection_weight(index),
                })
                .collect(),
        })
    }

    struct ConnectionsTestConfiguration {
        input_neuron_count: usize,
        stem_neuron_count: usize,
        output_neuron_count: usize,
        expected_connections: Vec<Connection>,
    }

    fn test_connections_and_neurons_are_generated_correctly(
        ConnectionsTestConfiguration {
            input_neuron_count,
            output_neuron_count,
            stem_neuron_count,
            expected_connections,
        }: ConnectionsTestConfiguration,
    ) {
        let neuron_count = input_neuron_count + stem_neuron_count + output_neuron_count;
        let input_neuron_connection_count = input_neuron_count * 2;
        let input_to_stem_neuron_connection_count = input_neuron_count.max(stem_neuron_count);
        let stem_to_output_neuron_connection_count = output_neuron_count.max(stem_neuron_count);
        let connection_count = input_neuron_connection_count
            + input_to_stem_neuron_connection_count
            + stem_to_output_neuron_connection_count;

        let random = {
            let mut random = RandomMock::new();
            random.expect_random_float_in_range_calls_in_order();

            for index in 0..connection_count {
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
                    partial_eq(output_neuron_count + 1),
                )
                .returns(stem_neuron_count);
            box random
        };
        let generator = CorpusCallosumClusterGeneGeneratorImpl::new(random);
        let corpus_callossum_configuration =
            corpus_callossum_configuration(input_neuron_count, output_neuron_count);
        let expected_neurons = vec![Neuron {}; neuron_count];
        let generated_cluster_gene =
            generator.generate_cluster_gene(&corpus_callossum_configuration);

        assert_eq!(expected_neurons, generated_cluster_gene.neurons);
        assert_eq!(expected_connections, generated_cluster_gene.connections);
    }

    fn connection_weight(index: usize) -> f64 {
        index as f64
    }

    fn corpus_callossum_configuration(
        input_neuron_count: usize,
        output_neuron_count: usize,
    ) -> CorpusCallosumConfiguration {
        CorpusCallosumConfiguration {
            input_neuron_count: NonZeroUsize::new(input_neuron_count).unwrap(),
            output_neuron_count: NonZeroUsize::new(output_neuron_count).unwrap(),
        }
    }
}
