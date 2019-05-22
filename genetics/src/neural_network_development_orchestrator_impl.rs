//! Default implementation of [`NeuralNetworkDevelopmentOrchestrator`].

pub use self::genome_deriver_impl::*;
pub use self::genome_mutator_impl::*;
pub use self::neural_network_configurator::NeuralNetworkConfiguratorImpl;
pub use self::neural_network_developer_impl::*;
use crate::*;
#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use myelin_neural_network::NeuralNetwork;
use nameof::{name_of, name_of_type};
use std::fmt::{self, Debug};
use std::rc::Rc;
use wonderbox::autoresolvable;

mod genome_deriver_impl;
mod genome_mutator_impl;
mod neural_network_configurator;
mod neural_network_developer_impl;

/// Trait for deriving a new [`Genome`] from two parent [`Genome`]s.
#[cfg_attr(test, mockable)]
pub trait GenomeDeriver: Debug + GenomeDeriverClone {
    /// Derives a new [`Genome`] from two parent [`Genome`]s.
    fn derive_genome_from_parents(&self, parent_genomes: (Genome, Genome)) -> Genome;
}

clone_box!(GenomeDeriver, GenomeDeriverClone);

/// Trait for mutating a [`Genome`].
#[cfg_attr(test, mockable)]
pub trait GenomeMutator: Debug + GenomeMutatorClone {
    /// Might apply mutations to any part of the genome.
    fn mutate_genome(&self, genome: Genome) -> Genome;
}

clone_box!(GenomeMutator, GenomeMutatorClone);

/// Provides a function that can be used to develop a neural network
#[cfg_attr(test, mockable)]
pub trait NeuralNetworkDeveloper: Debug {
    /// Develops a neural network and writes it into a [`NeuralNetworkConfigurator`].
    fn develop_neural_network(self: Box<Self>, configurator: &mut dyn NeuralNetworkConfigurator);
}

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait NeuralNetworkConfigurator: Debug {
    /// Adds a new unconnected neuron to the network
    fn push_neuron(&mut self) -> Handle;

    /// Adds a new unconnected neuron to the network and marks is as an input
    fn push_input_neuron(&mut self) -> Handle;

    /// Adds a new unconnected neuron to the network and marks is as an input
    fn push_output_neuron(&mut self) -> Handle;

    /// Adds a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    fn add_connection(&mut self, connection: Connection) -> Result<(), ()>;
}

/// A factory for building a [`NeuralNetwork`]
///
/// [`NeuralNetwork`]: ../../../myelin-neural-network/trait.NeuralNetwork.html
pub type NeuralNetworkFactory = dyn Fn() -> Box<dyn NeuralNetwork>;

/// Creates a new [`NeuralNetworkDeveloper`]
pub type NeuralNetworkDeveloperFactory = dyn for<'a> Fn(
    &'a NeuralNetworkDevelopmentConfiguration,
    &'a Genome,
) -> Box<dyn NeuralNetworkDeveloper + 'a>;

/// List of input neuron handles
pub type InputNeuronHandles = Vec<Handle>;

/// List of output neuron handles
pub type OutputNeuronHandles = Vec<Handle>;

/// Creates a new [`NeuralNetworkConfigurator`]
pub type NeuralNetworkConfiguratorFactory =
    dyn for<'a> Fn(
        &'a mut dyn NeuralNetwork,
        &'a mut InputNeuronHandles,
        &'a mut OutputNeuronHandles,
    ) -> Box<dyn NeuralNetworkConfigurator + 'a>;

/// Default implementation of a [`NeuralNetworkDevelopmentOrchestrator`]
#[derive(Clone)]
pub struct NeuralNetworkDevelopmentOrchestratorImpl {
    neural_network_factory: Rc<NeuralNetworkFactory>,
    neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
    neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory>,
    genome_deriver: Box<dyn GenomeDeriver>,
    genome_mutator: Box<dyn GenomeMutator>,
}

#[autoresolvable]
impl NeuralNetworkDevelopmentOrchestratorImpl {
    /// Constructs a new [`NeuralNetworkDevelopmentOrchestratorImpl`]
    pub fn new(
        neural_network_factory: Rc<NeuralNetworkFactory>,
        neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
        neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory>,
        genome_deriver: Box<dyn GenomeDeriver>,
        genome_mutator: Box<dyn GenomeMutator>,
    ) -> Self {
        Self {
            neural_network_factory,
            neural_network_developer_factory,
            neural_network_configurator_factory,
            genome_deriver,
            genome_mutator,
        }
    }
}

impl Debug for NeuralNetworkDevelopmentOrchestratorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(NeuralNetworkDevelopmentOrchestratorImpl))
            .field(name_of!(genome_deriver in Self), &self.genome_deriver)
            .field(name_of!(genome_mutator in Self), &self.genome_mutator)
            .finish()
    }
}

impl NeuralNetworkDevelopmentOrchestrator for NeuralNetworkDevelopmentOrchestratorImpl {
    fn develop_neural_network(
        &self,
        configuration: &NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork {
        let genome = self.create_genome(configuration);

        let mut neural_network = (self.neural_network_factory)();
        let mut input_neuron_handles = Vec::new();
        let mut output_neuron_handles = Vec::new();

        {
            let mut neural_network_configurator = (self.neural_network_configurator_factory)(
                &mut *neural_network,
                &mut input_neuron_handles,
                &mut output_neuron_handles,
            );
            let neural_network_developer =
                (self.neural_network_developer_factory)(configuration, &genome);
            neural_network_developer.develop_neural_network(&mut *neural_network_configurator);
        }

        DevelopedNeuralNetwork {
            genome,
            neural_network,
            input_neuron_handles,
            output_neuron_handles,
        }
    }
}

impl NeuralNetworkDevelopmentOrchestratorImpl {
    fn create_genome(&self, configuration: &NeuralNetworkDevelopmentConfiguration) -> Genome {
        let genome = self
            .genome_deriver
            .derive_genome_from_parents(configuration.parent_genomes.clone());
        self.genome_mutator.mutate_genome(genome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::*;
    use crate::neural_network_development_orchestrator_impl::{
        GenomeDeriverMock, GenomeMutatorMock,
    };
    use mockiato::{any, partial_eq};
    use myelin_neural_network::NeuralNetworkMock;

    #[test]
    fn orchestrates_neural_network_development() {
        let parent_genome_one = create_genome_with_single_hox_gene(1);
        let parent_genome_two = create_genome_with_single_hox_gene(2);
        let merged_genome = create_genome_with_single_hox_gene(3);
        let mutated_genome = create_genome_with_single_hox_gene(4);

        let development_configuration = NeuralNetworkDevelopmentConfiguration {
            parent_genomes: (parent_genome_one.clone(), parent_genome_two.clone()),
            input_neuron_count: NonZeroUsize::new(1).unwrap(),
            output_neuron_count: NonZeroUsize::new(1).unwrap(),
        };

        let neural_network_factory: Rc<NeuralNetworkFactory> = Rc::new(|| {
            let neural_network = NeuralNetworkMock::new();
            box neural_network
        });

        let neural_network_configurator_factory: Rc<NeuralNetworkConfiguratorFactory> =
            Rc::new(|_, _, _| {
                let neural_network_configurator = NeuralNetworkConfiguratorMock::new();
                box neural_network_configurator
            });

        let neural_network_builder_factory: Rc<NeuralNetworkDeveloperFactory> = {
            let development_configuration = development_configuration.clone();
            let mutated_genome = mutated_genome.clone();

            Rc::new(move |configuration, genome| {
                assert_eq!(development_configuration, *configuration);
                assert_eq!(&mutated_genome, genome);

                let mut neural_network_developer = NeuralNetworkDeveloperMock::new();
                neural_network_developer.expect_develop_neural_network(any());
                box neural_network_developer
            })
        };

        let mut genome_deriver = GenomeDeriverMock::new();
        genome_deriver
            .expect_derive_genome_from_parents(partial_eq((parent_genome_one, parent_genome_two)))
            .times(1)
            .returns(merged_genome.clone());

        let mut genome_mutator = GenomeMutatorMock::new();
        genome_mutator
            .expect_mutate_genome(partial_eq(merged_genome))
            .times(1)
            .returns(mutated_genome.clone());

        let orchestrator = NeuralNetworkDevelopmentOrchestratorImpl::new(
            neural_network_factory,
            neural_network_builder_factory,
            neural_network_configurator_factory,
            box genome_deriver,
            box genome_mutator,
        );

        let developed_neural_network =
            orchestrator.develop_neural_network(&development_configuration);

        assert_eq!(mutated_genome, developed_neural_network.genome);
    }

    fn create_genome_with_single_hox_gene(cluster_gene: usize) -> Genome {
        Genome {
            hox_genes: vec![HoxGene {
                placement_target: HoxPlacement::Standalone,
                cluster_gene: ClusterGeneIndex(cluster_gene),
                disabled_connections: Vec::new(),
            }],
            ..Genome::default()
        }
    }
}
