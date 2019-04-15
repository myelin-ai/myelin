use super::*;

#[test]
fn places_nothing_when_hox_gene_points_to_non_existent_cluster_gene() {
    let mut genome = genome_stub();
    add_initial_hox_gene_to_genome(&mut genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    developer.develop_neural_network(&mut configurator);
}
