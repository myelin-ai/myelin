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

fn cluster_with_input_neuron_marker_can_be_attached_to_initial_cluster() {
    unimplemented!()
}

fn input_neuron_marker_is_respected() {
    unimplemented!()
}

fn cluster_with_output_neuron_marker_can_be_attached_to_initial_cluster() {
    unimplemented!()
}

fn output_neuron_marker_is_respected() {
    unimplemented!()
}

impl GenomeStubBuilder {
    fn add_first_cluster_marked_as_initial_cluster(&mut self) -> &mut Self {
        self.add_first_cluster_with_specialization(ClusterGeneSpecilization::Initial)
    }

    fn add_second_cluster_marked_as_input_cluster(&mut self) -> &mut Self {
        self.add_first_cluster_with_specialization(ClusterGeneSpecilization::Input(
            NeuronClusterLocalIndex(2),
        ))
    }
}
