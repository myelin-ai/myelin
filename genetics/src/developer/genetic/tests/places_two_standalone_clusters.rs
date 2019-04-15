use super::*;

#[test]
fn places_two_standalone_clusters() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);
    let genome = add_initial_hox_gene_to_genome(genome);
    let genome = add_initial_hox_gene_to_genome(genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 8);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_standalone(&mut configurator, 4);

    developer.develop_neural_network(&mut configurator);
}
