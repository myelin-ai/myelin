use crate::neural_network::*;

/// A spiking neuron
#[derive(Debug)]
pub struct SpikingNeuron;

impl SpikingNeuron {
    /// Constructs a new neuron
    pub fn new() -> Self {
        SpikingNeuron
    }

    /// The step function, called every smallest tick in the simulation
    pub fn step(
        &mut self,
        _time_since_last_step: TimeInMilliseconds,
        _inputs: MembranePotential,
    ) -> MembranePotential {
        MembranePotential(0.0)
    }
}

impl Default for SpikingNeuron {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_constructed() {
        let _neuron = SpikingNeuron::default();
    }
}
