use super::*;

#[test]
fn errors_if_hox_gene_does_not_exist() {
    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };
    let expected_genome = genome.clone();

    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(2),
        new_target_neuron: ClusterNeuronIndex(1),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn errors_if_hox_gene_has_standalone_placement() {
    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };
    let expected_genome = genome.clone();

    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(1),
        new_target_neuron: ClusterNeuronIndex(1),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn change_target_neuron() {
    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            (|| {
                let mut gene = standalone_hox_gene(ClusterGeneIndex(1));

                gene.placement_target = HoxPlacement::ClusterGene {
                    cluster_gene: ClusterGeneIndex(1),
                    target_neuron: ClusterNeuronIndex(1),
                };

                gene
            })(),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    let expected_genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            (|| {
                let mut gene = standalone_hox_gene(ClusterGeneIndex(1));

                gene.placement_target = HoxPlacement::ClusterGene {
                    cluster_gene: ClusterGeneIndex(1),
                    target_neuron: ClusterNeuronIndex(2),
                };

                gene
            })(),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    let mutation = Mutation::ChangeTargetNeuron {
        hox_gene: HoxGeneIndex(1),
        new_target_neuron: ClusterNeuronIndex(2),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}
