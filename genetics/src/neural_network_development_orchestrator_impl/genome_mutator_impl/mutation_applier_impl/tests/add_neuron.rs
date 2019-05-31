use super::*;

#[test]
fn errors_when_cluster_gene_does_not_exist() {
    let genome = empty_genome().add_first_cluster_gene();
    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(1),
        connection_index: ClusterConnectionIndex(0),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn errors_when_connection_does_not_exist() {
    let genome = empty_genome().add_first_cluster_gene();
    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(0),
        connection_index: ClusterConnectionIndex(2),
    };
    test_mutation_application(MutationApplicationTestParameters {
        expected_genome: genome.clone(),
        genome,
        mutation,
        result_test_fn: Result::is_err,
    });
}

#[test]
fn creates_new_neuron_and_new_connection() {
    const NEW_CONNECTION_WEIGHT: f64 = 0.125;

    let genome = empty_genome().add_first_cluster_gene();

    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(0),
        connection_index: ClusterConnectionIndex(0),
    };

    let first_cluster_gene = first_cluster_gene();
    let expected_neuron_count = first_cluster_gene.neurons.len() + 1;
    let expected_cluster_gene = ClusterGene {
        neurons: vec![Neuron {}; expected_neuron_count],
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(2),
                weight: 1.0,
            },
            first_cluster_gene.connections[1],
            Connection {
                from: ClusterNeuronIndex(2),
                to: ClusterNeuronIndex(1),
                weight: NEW_CONNECTION_WEIGHT,
            },
        ],
        ..first_cluster_gene
    };
    let expected_genome = empty_genome().add_cluster_gene(expected_cluster_gene);

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}
