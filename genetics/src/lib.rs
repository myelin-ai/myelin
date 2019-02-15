//! Genes, genomes and the mechanisms needed to evolve neural networks from them

#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use myelin_neural_network::NeuralNetwork;

/// The set of all genes in an organism
#[derive(Debug)]
pub struct Genome;

/// A factory for producing a [`NeuralNetwork`] out of a [`Genome`]
///
/// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
/// [`Genome`]: ./struct.Genome.html
pub trait NeuralNetworkDeveloper {
    /// Create a [`NeuralNetwork`] out of a [`Genome`]
    ///
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    /// [`Genome`]: ./struct.Genome.html
    fn develop_neural_network(genome: Genome) -> Box<dyn NeuralNetwork>;
}
