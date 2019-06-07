use super::*;

#[test]
fn adds_single_bridge() {
    let initial_target_neuron_on_mutation_target = ClusterNeuronIndex(2);
    let base_genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)));

    let genome = base_genome.clone().add_hox_gene(hox_placed_on_hox_gene(
        ClusterGeneIndex(1),
        HoxGeneIndex(0),
        initial_target_neuron_on_mutation_target,
    ));

    let bridge_cluster = cluster_gene();

    let bridge_hox = HoxGene {
        placement_target: HoxPlacement::HoxGene {
            hox_gene: HoxGeneIndex(0),
            target_neuron: initial_target_neuron_on_mutation_target,
        },
        cluster_gene: ClusterGeneIndex(2),
        disabled_connections: Vec::new(),
    };

    let expected_genome = base_genome
        .add_cluster_gene(bridge_cluster.clone())
        .add_hox_gene(hox_placed_on_hox_gene(
            ClusterGeneIndex(1),
            HoxGeneIndex(2),
            new_target_neuron(),
        ))
        .add_hox_gene(bridge_hox);

    let mutation = Mutation::Bridge {
        target_hox_gene: HoxGeneIndex(1),
        bridge_cluster_gene: bridge_cluster,
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

#[test]
fn adds_bridge_to_neuron_with_multiple_attached_clusters() {
    let initial_target_neuron_on_mutation_target = ClusterNeuronIndex(2);
    let base_genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)));

    let genome = base_genome.clone().add_hox_gene(hox_placed_on_hox_gene(
        ClusterGeneIndex(1),
        HoxGeneIndex(0),
        initial_target_neuron_on_mutation_target,
    ));

    let genome = base_genome.clone().add_hox_gene(hox_placed_on_hox_gene(
        ClusterGeneIndex(1),
        HoxGeneIndex(0),
        initial_target_neuron_on_mutation_target,
    ));

    let bridge_cluster = cluster_gene();

    let bridge_hox = HoxGene {
        placement_target: HoxPlacement::HoxGene {
            hox_gene: HoxGeneIndex(0),
            target_neuron: initial_target_neuron_on_mutation_target,
        },
        cluster_gene: ClusterGeneIndex(2),
        disabled_connections: Vec::new(),
    };

    let expected_genome = base_genome
        .add_cluster_gene(bridge_cluster.clone())
        .add_hox_gene(hox_placed_on_hox_gene(
            ClusterGeneIndex(1),
            HoxGeneIndex(3),
            new_target_neuron(),
        ))
        .add_hox_gene(bridge_hox);

    let mutation = Mutation::Bridge {
        target_hox_gene: HoxGeneIndex(1),
        bridge_cluster_gene: bridge_cluster,
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

#[test]
fn adds_multiple_bridges() {
    let initial_target_neuron_on_mutation_target = ClusterNeuronIndex(2);

    let base_genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)))
        .add_hox_gene(hox_placed_on_hox_gene(
            ClusterGeneIndex(0),
            HoxGeneIndex(0),
            // Irrelevant for test
            ClusterNeuronIndex(1),
        ));

    let genome = base_genome.clone().add_hox_gene(hox_placed_on_cluster_gene(
        ClusterGeneIndex(1),
        ClusterGeneIndex(0),
        initial_target_neuron_on_mutation_target,
    ));

    let bridge_cluster = cluster_gene();

    let bridge_hox = HoxGene {
        placement_target: HoxPlacement::ClusterGene {
            cluster_gene: ClusterGeneIndex(0),
            target_neuron: initial_target_neuron_on_mutation_target,
        },
        cluster_gene: ClusterGeneIndex(2),
        disabled_connections: Vec::new(),
    };

    let expected_genome = base_genome
        .add_cluster_gene(bridge_cluster.clone())
        .add_hox_gene(hox_placed_on_cluster_gene(
            ClusterGeneIndex(1),
            ClusterGeneIndex(2),
            new_target_neuron(),
        ))
        .add_hox_gene(bridge_hox);

    let mutation = Mutation::Bridge {
        target_hox_gene: HoxGeneIndex(2),
        bridge_cluster_gene: bridge_cluster,
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 3],
        ..empty_cluster_gene()
    }
}

fn new_target_neuron() -> ClusterNeuronIndex {
    ClusterNeuronIndex(1)
}
