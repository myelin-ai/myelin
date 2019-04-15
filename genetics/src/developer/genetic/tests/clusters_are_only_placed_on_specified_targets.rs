use super::*;

#[test]
fn clusters_are_only_placed_on_specified_targets() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);
    let genome = add_second_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome = add_hox_gene_placing_first_cluster_on_first_hox(genome);
    let genome = add_hox_gene_placing_second_cluster_on_first_hox(genome);
    let genome = add_hox_gene_placing_second_cluster_on_first_cluster(genome);

    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_second_cluster_placed_on_first_hox_by_third_hox(&mut configurator);
    expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(&mut configurator);
    expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

#[test]
fn clusters_are_only_placed_on_specified_targets_when_target_is_hox_that_targeted_cluster_type() {
    let genome = genome_stub();
    let genome = add_first_cluster_to_genome(genome);
    let genome = add_second_cluster_to_genome(genome);

    let genome = add_initial_hox_gene_to_genome(genome);
    let genome = add_hox_gene_placing_first_cluster_on_first_hox(genome);
    let genome = add_hox_gene_placing_second_cluster_on_first_hox(genome);
    let genome = add_hox_gene_placing_second_cluster_on_first_cluster(genome);
    let genome = add_hox_gene_placing_first_cluster_on_fourth_hox(genome);
    let config = config_stub();

    let developer = box GeneticNeuralNetworkDeveloper::new(config, genome);
    let mut configurator = NeuralNetworkConfiguratorMock::new();

    expect_push_amount_of_neurons(&mut configurator, 10);
    expect_first_cluster_placed_standalone(&mut configurator, 0);
    expect_first_cluster_placed_on_first_hox_by_second_hox(&mut configurator);
    expect_second_cluster_placed_on_first_hox_by_third_hox(&mut configurator);

    expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(&mut configurator);
    expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(&mut configurator);

    expect_first_cluster_placed_on_fourth_placed_cluster_by_fifth_hox(&mut configurator);
    expect_first_cluster_placed_on_fifth_placed_cluster_by_fifth_hox(&mut configurator);

    developer.develop_neural_network(&mut configurator);
}

fn add_hox_gene_placing_first_cluster_on_first_hox(mut genome: Genome) -> Genome {
    genome.hox_genes.push(HoxGene {
        placement: HoxPlacement::HoxGene {
            hox_gene: HoxGeneIndex(0),
            target_neuron: NeuronClusterLocalIndex(3),
        },
        cluster_index: ClusterGeneIndex(0),
        disabled_connections: Vec::new(),
    });
    genome
}

fn add_hox_gene_placing_second_cluster_on_first_hox(mut genome: Genome) -> Genome {
    genome.hox_genes.insert(
        1,
        HoxGene {
            placement: HoxPlacement::HoxGene {
                hox_gene: HoxGeneIndex(0),
                target_neuron: NeuronClusterLocalIndex(2),
            },
            cluster_index: ClusterGeneIndex(1),
            disabled_connections: Vec::new(),
        },
    );
    genome
}

fn add_hox_gene_placing_first_cluster_on_fourth_hox(mut genome: Genome) -> Genome {
    genome.hox_genes.insert(
        1,
        HoxGene {
            placement: HoxPlacement::HoxGene {
                hox_gene: HoxGeneIndex(3),
                target_neuron: NeuronClusterLocalIndex(1),
            },
            cluster_index: ClusterGeneIndex(0),
            disabled_connections: Vec::new(),
        },
    );

    genome
}

fn expect_first_cluster_placed_on_first_hox_by_second_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 4, 1, 3)
}

fn expect_second_cluster_placed_on_first_hox_by_third_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 7, 0, 2)
}

fn expect_second_cluster_placed_on_first_placed_cluster_by_fourth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 9, 0, 2)
}

fn expect_second_cluster_placed_on_second_placed_cluster_by_fourth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 11, 0, 5)
}

fn expect_first_cluster_placed_on_fourth_placed_cluster_by_fifth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 13, 1, 9)
}

fn expect_first_cluster_placed_on_fifth_placed_cluster_by_fifth_hox(
    configurator: &mut NeuralNetworkConfiguratorMock<'_>,
) {
    expect_first_cluster_placed_on_hox()(configurator, 16, 1, 11)
}
