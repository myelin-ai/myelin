//! Default implementation of [`NeuralNetworkDevelopmentOrchestrator`].

use crate::deriver::GenomeDeriver;
use crate::mutator::GenomeMutator;
use crate::*;
#[cfg(test)]
use mockiato::mockable;
use nameof::{name_of, name_of_type};
use std::fmt::{self, Debug};
use std::rc::Rc;

/// Provides a function that can be used to develop a neural network
#[cfg_attr(test, mockable)]
pub trait NeuralNetworkDeveloper: Debug {
    /// Develops a neural network and writes it into a [`NeuralNetworkConfigurator`].
    fn develop_neural_network(self: Box<Self>, builder: &mut NeuralNetworkBuilder);
}

/// Configuration storage for a [`NeuralNetworkDeveloper`].
#[derive(Debug)]
pub struct NeuralNetworkBuilder {}

impl NeuralNetworkBuilder {
    /// Creates a new [`NeuralNetworkBuilder`] for a [`DevelopedNeuralNetwork`]
    pub fn new(_developed_neural_network: DevelopedNeuralNetwork) -> Self {
        unimplemented!()
    }

    /// Adds a new unconnected neuron to the network
    pub fn push_neuron(&mut self) -> Handle {
        unimplemented!();
    }

    /// Adds a new connection between two neurons.
    /// # Errors
    /// Returns `Err` if an involved handle is invalid
    pub fn add_connection(&mut self, _connection: Connection) -> Result<(), ()> {
        unimplemented!();
    }

    /// Marks a neuron as an input
    pub fn mark_neuron_as_input(&mut self, _handle: Handle) -> Result<(), ()> {
        unimplemented!();
    }

    /// Marks a neuron as an output
    pub fn mark_neuron_as_output(&mut self, _handle: Handle) -> Result<(), ()> {
        unimplemented!();
    }

    /// Consumes `self`, returning the built [`DevelopedNeuralNetwork`]
    pub fn build(self) -> DevelopedNeuralNetwork {
        unimplemented!();
    }
}

/// A factory for building a [`NeuralNetwork`]
///
/// [`NeuralNetwork`]: ../../../myelin-neural-network/trait.NeuralNetwork.html
pub type NeuralNetworkFactory = dyn Fn() -> Box<dyn NeuralNetwork>;

/// Creates a new [`NeuralNetworkDeveloper`]
pub type NeuralNetworkDeveloperFactory = dyn for<'a> Fn(
    &'a NeuralNetworkDevelopmentConfiguration,
    Genome,
) -> Box<dyn NeuralNetworkDeveloper + 'a>;

/// Default implementation of a [`NeuralNetworkDevelopmentOrchestrator`]
#[derive(Clone)]
pub struct NeuralNetworkDevelopmentOrchestratorImpl {
    neural_network_factory: Rc<NeuralNetworkFactory>,
    neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
    genome_deriver: Box<dyn GenomeDeriver>,
    genome_mutator: Box<dyn GenomeMutator>,
}

impl NeuralNetworkDevelopmentOrchestratorImpl {
    /// Constructs a new [`NeuralNetworkDevelopmentOrchestratorImpl`]
    pub fn new(
        neural_network_factory: Rc<NeuralNetworkFactory>,
        neural_network_developer_factory: Rc<NeuralNetworkDeveloperFactory>,
        genome_deriver: Box<dyn GenomeDeriver>,
        genome_mutator: Box<dyn GenomeMutator>,
    ) -> Self {
        Self {
            neural_network_factory,
            neural_network_developer_factory,
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
        _neural_network_development_configuration: &NeuralNetworkDevelopmentConfiguration,
    ) -> DevelopedNeuralNetwork {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deriver::GenomeDeriverMock;
    use crate::genome::*;
    use crate::mutator::GenomeMutatorMock;
    use mockiato::{any, partial_eq};
    use myelin_neural_network::NeuralNetworkMock;

    #[test]
    fn develop_neural_network_works() {
        let parent_genome_one = create_genome_with_single_hox_gene(1);
        let parent_genome_two = create_genome_with_single_hox_gene(2);
        let merged_genome = create_genome_with_single_hox_gene(3);
        let mutated_genome = create_genome_with_single_hox_gene(4);

        let development_configuration = NeuralNetworkDevelopmentConfiguration {
            parent_genomes: (parent_genome_one.clone(), parent_genome_two.clone()),
            input_neuron_count: 1,
            output_neuron_count: 1,
        };

        let neural_network_factory: Rc<NeuralNetworkFactory> = Rc::new(|| {
            let neural_network = NeuralNetworkMock::new();
            box neural_network
        });

        let neural_network_builder_factory: Rc<NeuralNetworkDeveloperFactory> = {
            let development_configuration = development_configuration.clone();
            let mutated_genome = mutated_genome.clone();

            Rc::new(move |configuration, genome| {
                assert_eq!(development_configuration, *configuration);
                assert_eq!(mutated_genome, genome);

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
            box genome_deriver,
            box genome_mutator,
        );

        let developed_neural_network =
            orchestrator.develop_neural_network(&development_configuration);

        assert_eq!(mutated_genome, developed_neural_network.genome);
    }

    fn create_genome_with_single_hox_gene(cluster_index: usize) -> Genome {
        Genome {
            hox_genes: vec![HoxGene {
                placement: HoxPlacement::Standalone,
                cluster_index: ClusterGeneIndex(cluster_index),
                disabled_connections: Vec::new(),
            }],
            ..Genome::default()
        }
    }
}
