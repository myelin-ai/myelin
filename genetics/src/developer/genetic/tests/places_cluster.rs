use super::*;

#[test]
fn places_cluster() {
    let mut genome = genome_stub();
    add_first_cluster_to_genome(&mut genome);
    add_second_cluster_to_genome(&mut genome);

    add_initial_hox_gene_to_genome(&mut genome);
    add_hox_gene_placing_second_cluster_on_first_cluster(&mut genome);

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 7);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_second_cluster_placed_on_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn expect_second_cluster_placed_on_first_cluster_connections(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_second_cluster_placed_on_hox()(configurator, 4, 0, 2);
}
