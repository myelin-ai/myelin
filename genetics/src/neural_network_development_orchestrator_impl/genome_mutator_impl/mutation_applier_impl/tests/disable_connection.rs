use super::*;
use maplit::hashset;

const CONNECTION: ClusterConnectionIndex = ClusterConnectionIndex(6);

#[test]
fn errors_if_hox_gene_does_not_exists() {
    let genome = empty_genome();
    let expected_genome = genome.clone();
    let mutation = Mutation::DisableConnection {
        hox_gene: HoxGeneIndex(0),
        connection: ClusterConnectionIndex(0),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_err,
    })
}

#[test]
fn adds_connection_to_disabled_connections() {
    let genome = empty_genome().add_hox_gene(hox_gene());

    let expected_hox_gene = hox_gene_with_disabled_connection();
    let expected_genome = empty_genome().add_hox_gene(expected_hox_gene);

    let mutation = Mutation::DisableConnection {
        hox_gene: HoxGeneIndex(0),
        connection: CONNECTION,
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    })
}

#[test]
fn does_not_disable_the_connection_if_already_disabled() {
    let genome = empty_genome().add_hox_gene(hox_gene_with_disabled_connection());
    let expected_genome = genome.clone();

    let mutation = Mutation::DisableConnection {
        hox_gene: HoxGeneIndex(0),
        connection: CONNECTION,
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome,
        genome,
        mutation,
        result_test_fn: Result::is_ok,
    })
}

fn hox_gene() -> HoxGene {
    standalone_hox_gene(ClusterGeneIndex(0))
}

fn hox_gene_with_disabled_connection() -> HoxGene {
    HoxGene {
        disabled_connections: hashset! {CONNECTION},
        ..hox_gene()
    }
}
