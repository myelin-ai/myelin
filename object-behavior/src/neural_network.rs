//! Neural networks and their components

mod connection;
pub use self::connection::*;

/// A handle to a [`Neuron`]
pub type Handle = usize;

/// A modifier of incoming spikes
pub type Weight = f64;
