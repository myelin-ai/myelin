//! Contains types for the full [`Genome`]

use std::collections::HashSet;

/// The index of a [`Neuron`] in a [`ClusterGene`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClusterNeuronIndex(pub usize);

/// The index of a [`HoxGene`] in a [`Genome`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HoxGeneIndex(pub usize);

/// The index of a [`ClusterGene`] in a [`Genome`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClusterGeneIndex(pub usize);

/// The index of a [`Connection`] in a [`ClusterGene`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClusterConnectionIndex(pub usize);

/// A neuron
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Neuron;

impl Neuron {
    /// Build a new Neuron
    pub fn new() -> Self {
        Neuron::default()
    }
}

/// Weight of a [`Connection`]
pub type Weight = f64;

/// Definition of the connection between two neurons.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Connection {
    /// The index of the neuron that will be used for the start of the connection
    pub from: ClusterNeuronIndex,
    /// The index of the neuron that will be used for the end of the connection
    pub to: ClusterNeuronIndex,
    /// The weight of the connection
    pub weight: Weight,
}

/// The definition of a cluster blueprint, defining the neurons, the neuron that will be attached
/// to the target when the cluster is placed, and the connections inside the cluster.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterGene {
    /// The neurons of the cluster
    pub neurons: Vec<Neuron>,

    /// The connections that will be placed when the cluster gene is used.
    /// They define the flow of information in the neural network.
    pub connections: Vec<Connection>,

    /// A neuron in this cluster gene. When this cluster is placed onto another cluster,
    /// instead of creating a new neuron, the target neuron is used. The target neuron is defined
    /// in the [`HoxPlacement`] of the [`HoxGene`] that defines the placement of this cluster.
    pub placement_neuron: ClusterNeuronIndex,

    /// Additional information about the cluster's responsibilities.
    pub specialization: ClusterGeneSpecialization,
}

/// Additional information about a the responsibilities of a placed [`ClusterGene`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClusterGeneSpecialization {
    /// A normal, good ol' cluster.
    None,

    /// The initial cluster placed by the first [standalone hox gene].
    /// It can only be placed once.
    ///
    /// [standalone hox gene]: enum.HoxPlacement.html#variant.Standalone
    Initial,

    /// The neuron at the specified index receives external input
    Input(ClusterNeuronIndex),

    /// The membrane potential of the neuron at the specified index serves as output for external behavior
    Output(ClusterNeuronIndex),
}

impl Default for ClusterGeneSpecialization {
    fn default() -> Self {
        ClusterGeneSpecialization::None
    }
}

/// Describes the placement behaviour of a [`HoxGene`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HoxPlacement {
    /// This hox gene's cluster will be placed once for each previously placed cluster of the given [`ClusterGene`].
    ClusterGene {
        /// Index of a [`ClusterGene`] in the [`Genome`].
        cluster_gene: ClusterGeneIndex,
        /// Index of a neuron in an already placed cluster. Counterpart of the [`ClusterGene::placement_neuron`].
        target_neuron: ClusterNeuronIndex,
    },
    /// This hox gene's cluster will be placed once for each previously placed cluster of the given [`HoxGene`].
    HoxGene {
        /// Index of a [`HoxGene`] in the [`Genome`].
        hox_gene: HoxGeneIndex,
        /// Index of a neuron in an already placed cluster. Counterpart of the [`ClusterGene::placement_neuron`].
        target_neuron: ClusterNeuronIndex,
    },
    /// The cluster of this [`HoxGene`] will be placed without connecting to another one.
    /// This is usually only used for the first [`HoxGene`].
    Standalone,
}

/// A gene defining the placement of neuron clusters.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HoxGene {
    /// The target(s) that the hox places its cluster(s) on
    pub placement_target: HoxPlacement,
    /// Index of the cluster that will be instantiated and placed.
    pub cluster_gene: ClusterGeneIndex,
    /// These connections, if existent, will not be enabled on the placed cluster.
    pub disabled_connections: HashSet<ClusterConnectionIndex>,
}

/// The set of all genes in an organism
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Genome {
    /// The hox genes of the genome
    pub hox_genes: Vec<HoxGene>,
    /// Clusters than can be placed by hox genes
    pub cluster_genes: Vec<ClusterGene>,
}
