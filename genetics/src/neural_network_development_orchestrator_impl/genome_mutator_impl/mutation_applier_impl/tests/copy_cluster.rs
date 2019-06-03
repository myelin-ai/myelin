use super::*;

#[test]
fn errors_when_source_cluster_gene_does_not_exist() {
    let genome = empty_genome().add_cluster_gene(empty_cluster_gene());
    let expected_genome = genome.clone();

    const INVALID_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(3);
    let mutation = Mutation::CopyCluster {
        source_cluster_gene: INVALID_CLUSTER_GENE,
        new_hox_gene: standalone_hox_gene(ClusterGeneIndex(0)),
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn copies_cluster_and_places_new_hox_gene() {
    const COPIED_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(1);
    let new_hox_gene = standalone_hox_gene(COPIED_CLUSTER_GENE);
    test_copies_cluster_and_places_new_hox_gene(new_hox_gene);
}

#[test]
fn copies_cluster_and_places_new_hox_gene_when_hox_gene_does_not_place_new_cluster() {
    let new_hox_gene = standalone_hox_gene(ClusterGeneIndex(0));
    test_copies_cluster_and_places_new_hox_gene(new_hox_gene);
}

fn test_copies_cluster_and_places_new_hox_gene(new_hox_gene: HoxGene) {
    let cluster_gene = empty_cluster_gene();
    let genome = empty_genome().add_cluster_gene(cluster_gene.clone());
    let expected_genome = genome
        .clone()
        .add_cluster_gene(cluster_gene)
        .add_hox_gene(new_hox_gene.clone());

    const EXISTING_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(0);
    let mutation = Mutation::CopyCluster {
        source_cluster_gene: EXISTING_CLUSTER_GENE,
        new_hox_gene,
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}
