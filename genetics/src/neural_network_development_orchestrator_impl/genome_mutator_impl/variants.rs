use crate::genome::{
    ClusterGene, ClusterGeneIndex, Connection, ConnectionFilter, HoxGene, NeuronClusterLocalIndex,
    Weight,
};

/// All possible mutations.
#[derive(Debug, Clone, PartialEq)]
pub enum MutationVariants {
    /// Add a neuron to an existing cluster. The neuron will be placed on an existing connection.  
    /// A → B becomes A → C → B, where A and B are existing neurons and C is the newly placed neuron.
    AddNeuron {
        /// Index of the cluster that will be mutated.
        cluster: ClusterGeneIndex,
        /// Index of the connection on the mutated cluster.
        connection_index: usize,
    },
    /// Add a new connection between a pair of neurons on an existing cluster.
    AddConnection {
        /// Index of the cluster that will be mutated.
        cluster: ClusterGeneIndex,
        /// The newly placed connection.
        connection: Connection,
    },
    /// Mark an existing connection as disabled.
    DisableConnection {
        /// Index of the hox gene that will be mutated.
        hox_index: usize,
        /// The newly created filter.
        connection_filter: ConnectionFilter,
    },
    /// Nudge the weight of an existing connection by a small delta value.
    NudgeWeight {
        /// Index of the cluster that will be mutated.
        cluster: ClusterGeneIndex,
        /// Index of the connection that will be mutated.
        connection_index: usize,
        /// The small shift in weight that will be added to the specified connection.
        weight_delta: Weight,
    },
    /// Change the neuron that is marked as the placement neuron on a cluster.
    ChangePlacementNeuron {
        /// Index of the cluster that will be mutated.
        cluster: ClusterGeneIndex,
        /// Index of the neuron that will be the new placement neuron.
        new_placement_neuron: NeuronClusterLocalIndex,
    },
    /// Add a new cluster and place it through a new hox gene.
    AddCluster {
        /// Specification of the new cluster.
        new_cluster: ClusterGene,
        /// Specification of the new hox gene that will place the new cluster.
        new_hox: HoxGene,
    },
    /// Create a copy of an existing cluster and place it through a new hox gene.
    CopyCluster {
        /// Index of the cluster that will be copied.
        source_cluster: ClusterGeneIndex,
        /// Specification of the new hox gene that will place the new cluster.
        new_hox: HoxGene,
    },
    /// Modify a hox gene so that it places an independent copy of the cluster it previously placed,
    /// allowing the new cluster to mutate independently from the initially used cluster.
    DesyncCluster {
        /// Index of the hox gene that will be mutated.
        hox_index: usize,
        /// Index of the cluster that will be copied.
        cluster: ClusterGeneIndex,
    },
    /// Add a new cluster in-between two clusters that share a neuron.
    Bridge {
        /// Index of the origin cluster.
        source_cluster: ClusterGeneIndex,
        /// Index of the destination cluster.
        target_cluster: ClusterGeneIndex,
        /// The shared neuron's index in the target cluster.
        target_neuron: NeuronClusterLocalIndex,
        /// Specification of the new cluster.
        bridge_cluster: ClusterGene,
    },
    /// Add a new hox gene that places an existing cluster.
    AddHoxWithExistingCluster {
        /// Specification of the new hox gene.
        new_hox: HoxGene,
    },
    /// Change the target neuron of a hox gene
    ChangeTargetNeuron {
        /// Index of the hox gene that will be mutated.
        hox_index: usize,
        /// Index of the neuron that will be the new target neuron.
        new_target_neuron: NeuronClusterLocalIndex,
    },
    /// Add a new hox gene with the same configuration as an already existing one.
    DuplicateHox {
        /// Index of the hox gene that will be duplicated.
        hox_index: usize,
    },
}
