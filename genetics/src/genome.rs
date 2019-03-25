//! Contains types for the full [`Genome`]

/// The index of a [`Neuron`] in a [`ClusterGene`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NeuronClusterLocalIndex(pub usize);

/// The index of a [`HoxGene`] in a [`Genome`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HoxGeneIndex(pub usize);

/// The index of a [`ClusterGene`] in a [`Genome`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClusterGeneIndex(pub usize);

/// A neuron
#[derive(Debug, Clone, PartialEq)]
pub struct Neuron {}

/// Weight of a [`Connection`]
pub type Weight = f64;

/// Definition of the connection between two neurons.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Connection {
    /// The index of the neuron that will be used for the start of the connection
    pub from: NeuronClusterLocalIndex,
    /// The index of the neuron that will be used for the end of the connection
    pub to: NeuronClusterLocalIndex,
    /// The weight of the connection
    pub weight: Weight,
}

/// The definition of a cluster blueprint, defining the neurons, the neuron that will be attached
/// to the target when the cluster is placed, and the connections inside the cluster.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterGene {
    /// The neurons of the cluster
    pub neurons: Vec<Neuron>,
    /// The connections of the cluster that will be placed, defining how information can be passed
    pub connections: Vec<Connection>,
    /// The neuron that will be taken from the cluster that it will be placed on.
    /// This means it will not generate a neuron during creation if there is a placement target.
    pub placement_neuron: NeuronClusterLocalIndex,
}

/// Describes the placement behaviour of a [`HoxGene`].
#[derive(Debug, Clone, PartialEq)]
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
        /// Index of the [`HoxGene`] in the [`Genome`].
        hox_gene_index: HoxGeneIndex,
        /// Index of a neuron in an already placed cluster. The neuron will be shared the
        /// [`ClusterGene::placement_neuron`] of the cluster that is currently being placed.
        target_neuron_index: NeuronClusterLocalIndex,
    },
    /// This hox places its cluster without attaching to another. Usually only used in the first hox.
    Standalone,
}

/// Possibly matches a [`Connection`]. See [`HoxGene`]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConnectionFilter {
    /// Equivalent of [`Connection::from`]
    pub from: NeuronClusterLocalIndex,
    /// Equivalent of [`Connection::to`]
    pub to: NeuronClusterLocalIndex,
}

impl From<Connection> for ConnectionFilter {
    fn from(Connection { from, to, .. }: Connection) -> ConnectionFilter {
        ConnectionFilter { from, to }
    }
}

/// A gene defining the placement of neuron clusters.
#[derive(Debug, Clone, PartialEq)]
pub struct HoxGene {
    /// Placement target of the hox
    pub placement: HoxPlacement,
    /// Index of the cluster that will be instantiated and placed.
    pub cluster_index: ClusterGeneIndex,
    /// These connections, if existent, will not be enabled on the placed cluster.
    pub disabled_connections: Vec<ConnectionFilter>,
}

/// The set of all genes in an organism
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genome {
    /// The hox genes of the genome
    pub hox_genes: Vec<HoxGene>,
    /// Clusters than can be placed by hox genes
    pub cluster_genes: Vec<ClusterGene>,
}
