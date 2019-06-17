use crate::genome::*;
use crate::genome_generator_impl::IoClusterGeneGenerator;
use myelin_random::Random;
use std::iter;

/// Default implementation of [`IoClusterGeneGenerator`]
#[derive(Debug)]
pub struct IoClusterGeneGeneratorImpl {
    random: Box<dyn Random>,
}

impl IoClusterGeneGeneratorImpl {
    /// Creates a new [`IoClusterGeneGeneratorImpl`].
    pub fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }
}

impl IoClusterGeneGenerator for IoClusterGeneGeneratorImpl {
    fn generate_input_cluster_gene(&self) -> ClusterGene {
        self.generate_cluster_gene(ClusterGeneSpecialization::Input(IO_NEURON))
    }

    fn generate_output_cluster_gene(&self) -> ClusterGene {
        self.generate_cluster_gene(ClusterGeneSpecialization::Output(IO_NEURON))
    }
}

impl IoClusterGeneGeneratorImpl {
    fn generate_cluster_gene(&self, specialization: ClusterGeneSpecialization) -> ClusterGene {
        let neuron_count = self
            .random
            .usize_in_range(MIN_NEURONS_PER_CLUSTER, MAX_NEURONS_PER_CLUSTER);
        let neurons = vec![Neuron {}; neuron_count];
        let connections = self.create_connections(neuron_count).collect();
        ClusterGene {
            placement_neuron: PLACEMENT_NEURON,
            neurons,
            connections,
            specialization,
        }
    }

    fn create_connections(&self, neuron_count: usize) -> impl Iterator<Item = Connection> + '_ {
        (0..neuron_count)
            .zip((0..neuron_count).skip(1))
            .map(move |(first_neuron_index, second_neuron_index)| {
                self.create_connections_between_two_neurons(first_neuron_index, second_neuron_index)
            })
            .flatten()
    }

    fn create_connections_between_two_neurons(
        &self,
        first_neuron_index: usize,
        second_neuron_index: usize,
    ) -> impl Iterator<Item = Connection> {
        let connection = self.create_connection(first_neuron_index, second_neuron_index);
        let reverse_connection = self.create_connection(second_neuron_index, first_neuron_index);

        iter::once(connection).chain(iter::once(reverse_connection))
    }

    fn create_connection(&self, from_index: usize, to_index: usize) -> Connection {
        let weight = self
            .random
            .f64_in_range(MIN_CONNECTION_WEIGHT, MAX_CONNECTION_WEIGHT);
        Connection {
            from: ClusterNeuronIndex(from_index),
            to: ClusterNeuronIndex(to_index),
            weight,
        }
    }
}

const MIN_NEURONS_PER_CLUSTER: usize = 2;
const PLACEMENT_NEURON: ClusterNeuronIndex = ClusterNeuronIndex(0);
const IO_NEURON: ClusterNeuronIndex = ClusterNeuronIndex(1);
/// Chosen arbitrarily
const MAX_NEURONS_PER_CLUSTER: usize = 12;
/// Chosen arbitrarily
const MIN_CONNECTION_WEIGHT: f64 = 0.000_000_1;
const MAX_CONNECTION_WEIGHT: f64 = 1.0;

#[cfg(test)]
mod tests {
    use super::*;
    use myelin_random::RandomMock;

    #[test]
    fn generates_random_number_of_neurons() {
        const NEURON_COUNT: usize = 5;
        const CONNECTION_COUNT: usize = (NEURON_COUNT - 1) * 2;

        let random: Box<dyn Random> = {
            let mut random = RandomMock::new();
            random
                .expect_usize_in_range(
                    |arg| arg.partial_eq(MIN_NEURONS_PER_CLUSTER),
                    |arg| arg.partial_eq(MAX_NEURONS_PER_CLUSTER),
                )
                .returns(NEURON_COUNT);

            random.expect_f64_in_range_calls_in_order();
            for index in 0..CONNECTION_COUNT {
                random
                    .expect_f64_in_range(
                        |arg| arg.partial_eq(MIN_CONNECTION_WEIGHT),
                        |arg| arg.partial_eq(MAX_CONNECTION_WEIGHT),
                    )
                    .returns(connection_weight(index));
            }
            box random
        };

        let cluster_gene_generator = IoClusterGeneGeneratorImpl::new(random);

        let expected_cluster_gene = ClusterGene {
            neurons: vec![Neuron {}; NEURON_COUNT],
            placement_neuron: PLACEMENT_NEURON,
            specialization: ClusterGeneSpecialization::Output(IO_NEURON),
            connections: vec![
                Connection {
                    from: ClusterNeuronIndex(0),
                    to: ClusterNeuronIndex(1),
                    weight: connection_weight(0),
                },
                Connection {
                    from: ClusterNeuronIndex(1),
                    to: ClusterNeuronIndex(0),
                    weight: connection_weight(1),
                },
                Connection {
                    from: ClusterNeuronIndex(1),
                    to: ClusterNeuronIndex(2),
                    weight: connection_weight(2),
                },
                Connection {
                    from: ClusterNeuronIndex(2),
                    to: ClusterNeuronIndex(1),
                    weight: connection_weight(3),
                },
                Connection {
                    from: ClusterNeuronIndex(2),
                    to: ClusterNeuronIndex(3),
                    weight: connection_weight(4),
                },
                Connection {
                    from: ClusterNeuronIndex(3),
                    to: ClusterNeuronIndex(2),
                    weight: connection_weight(5),
                },
                Connection {
                    from: ClusterNeuronIndex(3),
                    to: ClusterNeuronIndex(4),
                    weight: connection_weight(6),
                },
                Connection {
                    from: ClusterNeuronIndex(4),
                    to: ClusterNeuronIndex(3),
                    weight: connection_weight(7),
                },
            ],
        };

        let cluster_gene = cluster_gene_generator.generate_output_cluster_gene();

        assert_eq!(expected_cluster_gene, cluster_gene)
    }

    fn connection_weight(index: usize) -> f64 {
        index as f64
    }
}
