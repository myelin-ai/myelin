use super::*;

#[test]
fn duplicate_hox_fails_if_hox_gene_does_not_exist() {
    let mutation = Mutation::DuplicateHox {
        hox_gene: HoxGeneIndex(2),
    };

    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 2],
    };

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn duplicates_hox_gene() {
    let mutation = Mutation::DuplicateHox {
        hox_gene: HoxGeneIndex(1),
    };

    let genome = Genome {
        hox_genes: vec![
            standalone_hox_gene(ClusterGeneIndex(0)),
            standalone_hox_gene(ClusterGeneIndex(1)),
            standalone_hox_gene(ClusterGeneIndex(2)),
        ],
        cluster_genes: vec![empty_cluster_gene(); 3],
    };

    let expected_genome = genome
        .clone()
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(1)));

    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}
