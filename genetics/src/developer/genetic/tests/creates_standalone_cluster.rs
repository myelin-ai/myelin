use super::*;

#[test]
fn creates_standalone_cluster() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);
    let genome = add_initial_hox_gene_to_genome(genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 4);
    expect_first_cluster_connections(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn expect_first_cluster_connections(configurator: &mut NeuralNetworkConfiguratorMock<'_>) {
    first_cluster_connections()
        .into_iter()
        .map(connection_definition_to_connection)
        .for_each(|connection| {
            configurator
                .expect_add_connection(partial_eq(connection))
                .returns(Ok(()));
        });
}
