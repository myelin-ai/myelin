use super::*;

#[test]
fn adds_bridge() {
    let shared_neuron = ClusterNeuronIndex(1);
    let genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)))
        .add_hox_gene(hox_placed_on_hox_gene(ClusterGeneIndex(0), shared_neuron));

    let expected_hox = HoxGene {
        placement_target: HoxPlacement::HoxGene {
            hox_gene: HoxGeneIndex(2),
            target_neuron: shared_neuron,
        },
        cluster_gene: ClusterGeneIndex(2),
        disabled_connections: Vec::new(),
    };

    let expected_genome = genome
        .clone()
        .add_cluster_gene(new_cluster_gene.clone())
        .add_hox_gene(expected_hox);

    let mutation = Mutation::Bridge {
        source_cluster_gene: ClusterGeneIndex(0),
        target_cluster_gene: ClusterGeneIndex(1),
        target_neuron: shared_neuron,
        bridge_cluster_gene: cluster_gene(),
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
