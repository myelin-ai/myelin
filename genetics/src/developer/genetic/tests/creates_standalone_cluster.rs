use super::*;

#[test]
fn creates_standalone_cluster() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);
    add_initial_hox_gene_to_genome(&mut genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 4);
    expect_first_cluster_placed_standalone(&mut configurator, 0);

    developer.develop_neural_network(&mut configurator);
}
