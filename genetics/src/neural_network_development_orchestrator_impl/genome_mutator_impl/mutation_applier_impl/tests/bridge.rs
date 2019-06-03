use super::*;

#[test]
fn adds_bridge() {
    let shared_neuron = ClusterNeuronIndex(1);
    let genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)))
        .add_hox_gene(hox_placed_on_hox_gene(
            ClusterGeneIndex(1),
            HoxGeneIndex(0),
            shared_neuron,
        ));

    let bridge_cluster = cluster_gene();

    let new_target_neuron: ClusterNeuronIndex(1);

    let bridge_hox = HoxGene {
        placement_target: HoxPlacement::HoxGene {
            hox_gene: HoxGeneIndex(0),
            target_neuron: shared_neuron,
        },
        cluster_gene: ClusterGeneIndex(2),
        disabled_connections: Vec::new(),
    };

    let expected_genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(bridge_cluster.clone())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)))
        .add_hox_gene(hox_placed_on_hox_gene(
            ClusterGeneIndex(1),
            HoxGeneIndex(2),
            new_target_neuron,
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

fn cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 3],
        ..empty_cluster_gene()
    }
}
