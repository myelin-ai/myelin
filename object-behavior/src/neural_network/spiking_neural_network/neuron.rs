use super::constant;
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
        MembranePotential(constant::RESTING_POTENTIAL)
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

    #[test]
    fn step_with_no_inputs_returns_resting_potential() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = TimeInMilliseconds(1.0);
        let inputs = MembranePotential(0.0);
        let membrane_potential = neuron.step(elapsed_time, inputs);
        assert_eq!(constant::RESTING_POTENTIAL, membrane_potential.0);
    }

    #[test]
    fn step_with_high_input_and_no_elapsed_time_returns_resting_potential() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = TimeInMilliseconds(0.0);
        let inputs = MembranePotential(constant::ACTION_POTENTIAL);
        let membrane_potential = neuron.step(elapsed_time, inputs);
        assert_eq!(constant::RESTING_POTENTIAL, membrane_potential.0);
    }
}
