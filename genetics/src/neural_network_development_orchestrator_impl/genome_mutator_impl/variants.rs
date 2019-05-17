use crate::genome::{
    ClusterGeneIndex, Connection, HoxGene, HoxPlacement, NeuronClusterLocalIndex, Weight,
};

/// The possible genotypic mutations
#[derive(Debug, Clone, PartialEq)]
pub enum MutationVariants {
    AddNeuron {
        cluster: ClusterGeneIndex,
        connection_index: usize,
    },
    AddConnection {
        cluster: ClusterGeneIndex,
        connection: Connection,
    },
    DisableConnection {
        hox_index: usize,
        connection_filter: ConnectionFilter,
    },
    NudgeWeight {
        cluster: ClusterGeneIndex,
        connection_index: usize,
        weight_delta: Weight,
    },
    RemoveNeuron {
        cluster: ClusterGeneIndex,
        neuron: NeuronClusterLocalIndex,
    },
    ChangePlacementNeuron {
        cluster: ClusterGeneIndex,
        new_placement_neuron: NeuronClusterLocalIndex,
    },
    AddCluster {
        new_cluster: ClusterGene,
        new_hox: HoxGene,
    },
    CopyCluster {
        source_cluster: ClusterGeneIndex,
        new_hox: HoxGene,
    },
    DesyncCluster {
        cluster: ClusterGeneIndex,
        hox_index: usize,
    },
    Bridge {
        source_cluster: ClusterGeneIndex,
        target_cluster: ClusterGeneIndex,
        bridge_cluster: ClusterGene,
    },
    AddHoxWithExistingCluster {
        new_hox: HoxGene,
    },
    ChangeTargetNeuron {
        hox_index: usize,
        new_target_neuron: NeuronClusterLocalIndex,
    },
    DuplicateHox {
        hox_index: usize,
    },
}
