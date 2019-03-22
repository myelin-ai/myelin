//! Contains types for the full [`Genome`]

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NeuronClusterLocalIndex(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HoxGeneIndex(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClusterGeneIndex(pub usize);

#[derive(Debug, Clone)]
pub struct Neuron {}

pub type Weight = f64;

#[derive(Debug, Copy, Clone)]
pub struct Connection {
    pub from: NeuronClusterLocalIndex,
    pub to: NeuronClusterLocalIndex,
    pub weight: Weight,
}

#[derive(Debug, Clone)]
pub struct ClusterGene {
    pub neurons: Vec<Neuron>,
    pub connections: Vec<Connection>,
    /// The neuron that will be taken from the cluster that it will be placed on.
    /// This means it will not generate a [`NeuronInstance`] during creation if there is a placement target.
    pub placement_neuron: NeuronClusterLocalIndex,
}

/// Describes the placement behaviour of a hox gene.
#[derive(Debug, Clone)]
pub enum HoxPlacement {
    /// This hox will place its cluster on all previously instantiated clusters of the given [`ClusterGene`].
    ClusterGene {
        /// Index of the [`ClusterGene`] in the [`Genome`].
        cluster_gene_index: ClusterGeneIndex,
        /// Index of a neuron in an already placed cluster. The neuron will be shared the
        /// [`ClusterGene::placement_neuron`] of the cluster that is currently being placed.
        target_neuron_index: NeuronClusterLocalIndex,
    },
    /// This hox will place its cluster on all cluster instances placed by the referenced [`HoxGene`].
    HoxGene {
        /// Index of the hox gene in the [`Genome`].
        hox_gene_index: HoxGeneIndex,
        /// Index of a neuron in an already placed cluster. The neuron will be shared the
        /// [`ClusterGene::placement_neuron`] of the cluster that is currently being placed.
        target_neuron_index: NeuronClusterLocalIndex,
    },
    /// This hox places its cluster without attaching to another. Usually only used in the first hox.
    Standalone,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConnectionFilter {
    pub from: NeuronClusterLocalIndex,
    pub to: NeuronClusterLocalIndex,
}

impl From<Connection> for ConnectionFilter {
    fn from(Connection { from, to, .. }: Connection) -> ConnectionFilter {
        ConnectionFilter { from, to }
    }
}

#[derive(Debug, Clone)]
pub struct HoxGene {
    pub placement: HoxPlacement,
    pub cluster_index: ClusterGeneIndex,
    /// Those connections will not be enabled on the placed cluster
    pub disabled_connections: Vec<ConnectionFilter>,
}

#[derive(Debug, Clone, Default)]
pub struct Genome {
    pub hox_genes: Vec<HoxGene>,
    pub cluster_genes: Vec<ClusterGene>,
}
