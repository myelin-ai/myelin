//! Genes, genomes and the mechanisms needed to evolve neural networks from them

#![feature(specialization)]
#![deny(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_docs,
    clippy::doc_markdown,
    clippy::unimplemented
)]

use myelin_neural_network::NeuralNetwork;
use std::fmt::Debug;

/// The set of all genes in an organism
#[derive(Debug, Clone)]
pub struct Genome;

/// A factory for producing a [`NeuralNetwork`] out of a [`Genome`]
///
/// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
/// [`Genome`]: ./struct.Genome.html
pub trait NeuralNetworkDeveloper: Debug + NeuralNetworkDeveloperClone {
    /// Create a [`NeuralNetwork`] out of a pair of parent [`Genome`]s
    ///
    /// [`NeuralNetwork`]: ../myelin-neural-network/trait.NeuralNetwork.html
    /// [`Genome`]: ./struct.Genome.html
    fn develop_neural_network(&self, parent_genomes: (Genome, Genome)) -> Box<dyn NeuralNetwork>;
}

/// Supertrait used to make sure that all implementors
/// of [`NeuralNetworkDeveloper`] are [`Clone`]. You don't need
/// to care about this type.
///
/// [`NeuralNetworkDeveloper`]: ./trait.NeuralNetworkDeveloper.html
/// [`Clone`]: https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html
#[doc(hidden)]
pub trait NeuralNetworkDeveloperClone {
    fn clone_box(&self) -> Box<dyn NeuralNetworkDeveloper>;
}

impl<T> NeuralNetworkDeveloperClone for T
where
    T: NeuralNetworkDeveloper + Clone + 'static,
{
    default fn clone_box(&self) -> Box<dyn NeuralNetworkDeveloper> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NeuralNetworkDeveloper> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
