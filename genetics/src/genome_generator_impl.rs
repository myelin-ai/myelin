//! Default implementation of [`GenomeGenerator`].

use crate::genome::*;
use crate::{GenomeGenerator, GenomeGeneratorConfiguration};
use myelin_random::Random;
use std::iter;

/// Default implementation of [`GenomeGenerator`].
#[derive(Debug, Default, Clone)]
pub struct GenomeGeneratorImpl;

impl GenomeGenerator for GenomeGeneratorImpl {
    fn generate_genome(&self, _configuration: &GenomeGeneratorConfiguration) -> Genome {
        unimplemented!();
    }
}

fn generate_sensor_cluster_gene(random: &dyn Random) -> ClusterGene {
    let neuron_count = random.random_usize_in_range(
        MIN_NEURONS_PER_SENSOR_CLUSTER,
        MAX_NEURONS_PER_SENSOR_CLUSTER,
    );
    let neurons = vec![Neuron {}; neuron_count];

    let connections = (0..neuron_count)
        .zip((0..neuron_count).skip(1))
        .map(|(from_index, to_index)| {
            let connection = create_sensor_cluster_gene_connection(random, from_index, to_index);
            let reverse_connection =
                create_sensor_cluster_gene_connection(random, to_index, from_index);

            iter::once(connection).chain(iter::once(reverse_connection))
        })
        .flatten()
        .collect();

    ClusterGene {
        placement_neuron: SENSOR_CLUSTER_PLACEMENT_NEURON,
        neurons,
        connections,
    }
}

fn create_sensor_cluster_gene_connection(
    random: &dyn Random,
    from_index: usize,
    to_index: usize,
) -> Connection {
    let weight = random.random_float_in_range(MIN_CONNECTION_WEIGHT, MAX_CONNECTION_WEIGHT);
    Connection {
        from: NeuronClusterLocalIndex(from_index),
        to: NeuronClusterLocalIndex(to_index),
        weight,
    }
}

/// - Neuron 0: Placement neuron
/// - Neuron 1: Sensor neuron
const MIN_NEURONS_PER_SENSOR_CLUSTER: usize = 2;
/// Chosen arbitrarily
const MAX_NEURONS_PER_SENSOR_CLUSTER: usize = 12;
/// Chosen arbitrarily
const MIN_CONNECTION_WEIGHT: f64 = 0.000_000_1;
const MAX_CONNECTION_WEIGHT: f64 = 1.0;
const SENSOR_CLUSTER_PLACEMENT_NEURON: NeuronClusterLocalIndex = NeuronClusterLocalIndex(0);

#[cfg(test)]
mod tests {
    use super::*;
    use mockiato::partial_eq;
    use myelin_random::RandomMock;

    #[test]
    fn generates_random_number_of_neurons() {
        let random: Box<dyn Random> = {
            let mut random = RandomMock::new();
            random
                .expect_random_usize_in_range(
                    partial_eq(MIN_NEURONS_PER_SENSOR_CLUSTER),
                    partial_eq(MAX_NEURONS_PER_SENSOR_CLUSTER),
                )
                .returns(5);
            random.expect_random_float_in_range_calls_in_order();

            for index in 0..=7 {
                random
                    .expect_random_float_in_range(
                        partial_eq(MIN_CONNECTION_WEIGHT),
                        partial_eq(MAX_CONNECTION_WEIGHT),
                    )
                    .returns(f64::from(index));
            }
            box random
        };

        let expected_cluster_gene = ClusterGene {
            neurons: vec![Neuron {}; 5],
            placement_neuron: SENSOR_CLUSTER_PLACEMENT_NEURON,
            connections: vec![
                Connection {
                    from: NeuronClusterLocalIndex(0),
                    to: NeuronClusterLocalIndex(1),
                    weight: 0.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(1),
                    to: NeuronClusterLocalIndex(0),
                    weight: 1.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(1),
                    to: NeuronClusterLocalIndex(2),
                    weight: 2.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(2),
                    to: NeuronClusterLocalIndex(1),
                    weight: 3.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(2),
                    to: NeuronClusterLocalIndex(3),
                    weight: 4.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(3),
                    to: NeuronClusterLocalIndex(2),
                    weight: 5.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(3),
                    to: NeuronClusterLocalIndex(4),
                    weight: 6.0,
                },
                Connection {
                    from: NeuronClusterLocalIndex(4),
                    to: NeuronClusterLocalIndex(3),
                    weight: 7.0,
                },
            ],
        };

        let cluster_gene = generate_sensor_cluster_gene(&*random);

        assert_eq!(expected_cluster_gene, cluster_gene)
    }
}
