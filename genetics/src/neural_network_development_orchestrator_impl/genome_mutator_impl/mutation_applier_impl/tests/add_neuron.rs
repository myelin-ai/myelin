use super::*;

const NEW_CONNECTION_WEIGHT: f64 = 0.125;

#[test]
fn errors_when_cluster_gene_does_not_exist() {
    let genome = empty_genome().add_cluster_gene(cluster_gene());
    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(1),
        connection_index: ClusterConnectionIndex(0),
        new_connection_weight: NEW_CONNECTION_WEIGHT,
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
    let genome = empty_genome().add_cluster_gene(cluster_gene());
    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(0),
        connection_index: ClusterConnectionIndex(2),
        new_connection_weight: NEW_CONNECTION_WEIGHT,
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
    let genome = empty_genome().add_cluster_gene(cluster_gene());

    let mutation = Mutation::AddNeuron {
        cluster: ClusterGeneIndex(0),
        connection_index: ClusterConnectionIndex(0),
        new_connection_weight: NEW_CONNECTION_WEIGHT,
    };

    let cluster_gene = cluster_gene();
    let expected_neuron_count = cluster_gene.neurons.len() + 1;
    let expected_cluster_gene = ClusterGene {
        neurons: vec![Neuron {}; expected_neuron_count],
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(2),
                weight: 1.0,
            },
            cluster_gene.connections[1],
            Connection {
                from: ClusterNeuronIndex(2),
                to: ClusterNeuronIndex(1),
                weight: NEW_CONNECTION_WEIGHT,
            },
        ],
        ..cluster_gene
    };
    let expected_genome = empty_genome().add_cluster_gene(expected_cluster_gene);

    test_mutation_application(MutationApplicationTestParameters {
        genome,
        expected_genome,
        mutation,
        result_test_fn: Result::is_ok,
    });
}

fn cluster_gene() -> ClusterGene {
    ClusterGene {
        neurons: vec![Neuron {}; 2],
        connections: vec![
            Connection {
                from: ClusterNeuronIndex(0),
                to: ClusterNeuronIndex(1),
                weight: 1.0,
            },
            Connection {
                from: ClusterNeuronIndex(1),
                to: ClusterNeuronIndex(0),
                weight: 1.0,
            },
        ],
        ..empty_cluster_gene()
    }
}
