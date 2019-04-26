use super::*;

#[test]
fn places_two_standalone_clusters() {
    let genome = GenomeStubBuilder::new()
        .add_first_cluster()
        .add_initial_hox_gene()
        .add_initial_hox_gene()
        .build();

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 8);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_standalone(&mut configurator, 4);

    developer.develop_neural_network(&mut configurator);
}
