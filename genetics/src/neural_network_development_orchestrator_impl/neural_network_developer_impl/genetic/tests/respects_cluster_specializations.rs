use super::*;

#[test]
fn cluster_with_initial_marker_can_be_placed() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster_marked_as_initial_cluster()
        .add_initial_hox_gene()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 4);
    expect_first_cluster_placed_standalone(&mut configurator, 0);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn cluster_with_input_neuron_marker_can_be_attached_to_initial_cluster() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster_marked_as_initial_cluster()
        .add_second_cluster_marked_as_input_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_second_cluster_on_first_cluster()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    let neuron_handles = NeuronHandles {
        regular: (0..=4).collect(),
        input: vec![5],
        output: vec![],
    };
    expect_push_different_kinds_of_neurons(&mut configurator, neuron_handles);

    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_second_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn cluster_with_output_neuron_marker_can_be_attached_to_initial_cluster() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster_marked_as_initial_cluster()
        .add_second_cluster_marked_as_output_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_second_cluster_on_first_cluster()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    let neuron_handles = NeuronHandles {
        regular: (0..=4).collect(),
        input: vec![],
        output: vec![5],
    };
    expect_push_different_kinds_of_neurons(&mut configurator, neuron_handles);

    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_second_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn input_and_output_clusters_can_be_attached_to_initial_cluster() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster_marked_as_initial_cluster()
        .add_second_cluster_marked_as_input_cluster()
        .add_third_cluster_marked_as_output_cluster()
        .add_initial_hox_gene()
        .add_hox_gene_placing_second_cluster_on_first_cluster()
        .add_hox_gene_placing_third_cluster_on_first_cluster()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    let neuron_handles = NeuronHandles {
        regular: (0..=4).chain(6..=7).collect(),
        input: vec![5],
        output: vec![8],
    };
    expect_push_different_kinds_of_neurons(&mut configurator, neuron_handles);

    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_second_cluster_placed_on_first_cluster_connections(&mut configurator);
    expect_third_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

impl GenomeStubBuilder {
    fn add_first_cluster_marked_as_initial_cluster(&mut self) -> &mut Self {
        self.add_first_cluster_with_specialization(ClusterGeneSpecialization::Initial)
    }

    fn add_second_cluster_marked_as_input_cluster(&mut self) -> &mut Self {
        self.add_second_cluster_with_specialization(ClusterGeneSpecialization::Input(
            ClusterNeuronIndex(2),
        ))
    }

    fn add_second_cluster_marked_as_output_cluster(&mut self) -> &mut Self {
        self.add_second_cluster_with_specialization(ClusterGeneSpecialization::Output(
            ClusterNeuronIndex(2),
        ))
    }

    fn add_third_cluster_marked_as_output_cluster(&mut self) -> &mut Self {
        self.add_third_cluster_with_specialization(ClusterGeneSpecialization::Output(
            ClusterNeuronIndex(2),
        ))
    }

    fn add_hox_gene_placing_third_cluster_on_first_cluster(&mut self) -> &mut Self {
        self.add_hox_gene_placing_cluster_on_cluster(ClusterOnClusterTestParameters {
            target_cluster_gene: ClusterGeneIndex(0),
            target_neuron: ClusterNeuronIndex(1),
            cluster_gene: ClusterGeneIndex(2),
        });
        self
    }
}

fn expect_second_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_second_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 4,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 0,
                handle: 2,
            }),
        },
    );
}

fn expect_third_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_third_cluster_placed_on_hox()(
        configurator,
        ExpectConnectionsParameters {
            cluster_offset: 6,
            placement_neuron: Some(PlacementNeuronTranslation {
                index: 3,
                handle: 1,
            }),
        },
    );
}
