use crate::genome::*;

/// All possible mutations.
#[derive(Debug, Clone, PartialEq)]
pub enum Mutation {
    /// Add a neuron to an existing [`ClusterGene`]. The neuron will be placed on an existing [`Connection`].  
    /// A → B becomes A → C → B, where A and B are existing neurons and C is the newly placed neuron.
    AddNeuron {
        /// Index of the [`ClusterGene`] that will be mutated.
        cluster_gene: ClusterGeneIndex,
        /// Index of the [`Connection`] on the mutated [`ClusterGene`].
        connection: ClusterConnectionIndex,
        /// Weight for the newly placed [`Connection`]. (C → B)
        new_connection_weight: Weight,
    },
    /// Add a new [`Connection`] between a pair of neurons on an existing [`ClusterGene`].
    AddConnection {
        /// Index of the [`ClusterGene`] that will be mutated.
        cluster_gene: ClusterGeneIndex,
        /// The newly placed [`Connection`].
        connection: Connection,
    },
    /// Mark an existing [`Connection`] as disabled.
    DisableConnection {
        /// Index of the [`HoxGene`] that will be mutated.
        hox_gene: HoxGeneIndex,
        /// The disabled [`Connection`].
        connection: ClusterConnectionIndex,
    },
    /// Nudge the weight of an existing [`Connection`] by a small delta value.
    NudgeWeight {
        /// Index of the [`ClusterGene`] that will be mutated.
        cluster_gene: ClusterGeneIndex,
        /// Index of the [`Connection`] that will be mutated.
        connection: ClusterConnectionIndex,
        /// The small shift in weight that will be added to the specified [`Connection`].
        weight_delta: Weight,
    },
    /// Change the neuron that is marked as the placement neuron on a [`ClusterGene`].
    ChangePlacementNeuron {
        /// Index of the [`ClusterGene`] that will be mutated.
        cluster_gene: ClusterGeneIndex,
        /// Index of the neuron that will be the new placement neuron.
        new_placement_neuron: ClusterNeuronIndex,
    },
    /// Add a new [`ClusterGene`] and place it through a new [`HoxGene`].
    AddNewCluster {
        /// Specification of the new [`ClusterGene`].
        cluster_gene: ClusterGene,
        /// Specification of the new [`HoxGene`] that will place the new [`ClusterGene`].
        hox_gene: HoxGene,
    },
    /// Create a copy of an existing [`ClusterGene`] and place it through a new [`HoxGene`].
    CopyCluster {
        /// Index of the [`ClusterGene`] that will be copied.
        source_cluster_gene: ClusterGeneIndex,
        /// Specification of the new [`HoxGene`] that will place the new [`ClusterGene`].
        new_hox_gene: HoxGene,
    },
    /// Allow a [`ClusterGene`] to mutate independently by turning it into a new [`ClusterGene`].
    DesyncCluster {
        /// Index of the [`HoxGene`] that will be mutated.
        hox_gene: HoxGeneIndex,
        /// Index of the [`ClusterGene`] that will be copied.
        cluster_gene: ClusterGeneIndex,
    },
    /// Add a new [`ClusterGene`] in between two clusters that share a neuron.
    Bridge {
        /// Index of the origin [`ClusterGene`].
        source_cluster_gene: ClusterGeneIndex,
        /// Index of the destination [`ClusterGene`].
        target_cluster_gene: ClusterGeneIndex,
        /// The shared neuron's index in the target [`ClusterGene`].
        target_neuron: ClusterNeuronIndex,
        /// Specification of the new [`ClusterGene`].
        bridge_cluster_gene: ClusterGene,
    },
    /// Add a new [`HoxGene`] that places an existing [`ClusterGene`].
    AddHoxWithExistingCluster {
        /// Specification of the new [`HoxGene`].
        new_hox_gene: HoxGene,
    },
    /// Change the target neuron of a [`HoxGene`].
    ChangeTargetNeuron {
        /// Index of the [`HoxGene`] that will be mutated.
        hox_gene: HoxGeneIndex,
        /// Index of the neuron that will be the new target neuron.
        new_target_neuron: ClusterNeuronIndex,
    },
    /// Add a new [`HoxGene`] to the end of the [`Genome`] with the same configuration as an already existing one.
    DuplicateHox {
        /// Index of the [`HoxGene`] that will be duplicated.
        hox_gene: HoxGeneIndex,
    },
}
