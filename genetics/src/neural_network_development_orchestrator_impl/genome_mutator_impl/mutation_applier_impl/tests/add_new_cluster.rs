use super::*;

#[test]
fn adds_cluster_and_hox_gene() {
    let new_cluster_gene = cluster_gene();
    let new_hox_gene = hox_gene(ClusterGeneIndex(1));
    test_adds_cluster_and_hox_gene(new_cluster_gene, new_hox_gene);
}

#[test]
fn adds_cluster_and_hox_gene_even_if_hox_does_not_place_new_cluster() {
    const EXISTING_CLUSTER_GENE: ClusterGeneIndex = ClusterGeneIndex(0);
    let new_cluster_gene = cluster_gene();
    let new_hox_gene = hox_gene(EXISTING_CLUSTER_GENE);
    test_adds_cluster_and_hox_gene(new_cluster_gene, new_hox_gene);
}

fn test_adds_cluster_and_hox_gene(new_cluster_gene: ClusterGene, new_hox_gene: HoxGene) {
    let genome = empty_genome()
        .add_cluster_gene(cluster_gene())
        .add_hox_gene(standalone_hox_gene(ClusterGeneIndex(0)));

    let expected_genome = genome
        .clone()
        .add_cluster_gene(new_cluster_gene.clone())
        .add_hox_gene(new_hox_gene.clone());

    let mutation = Mutation::AddNewCluster {
        hox_gene: new_hox_gene,
        cluster_gene: new_cluster_gene,
    };

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn hox_gene(cluster_gene: ClusterGeneIndex) -> HoxGene {
    standalone_hox_gene(cluster_gene)
}

fn cluster_gene() -> ClusterGene {
    empty_cluster_gene()
}
