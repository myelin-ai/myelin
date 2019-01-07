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

    /// Update the state of the neuron and return it
    pub fn step(
        &mut self,
        _time_since_last_step: Milliseconds,
        _inputs: &[(MembranePotential, Weight)],
    ) -> Option<MembranePotential> {
        None
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
    fn emits_no_potential_without_inputs() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(1.0);
        let membrane_potential = neuron.step(elapsed_time, &[]);
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_with_high_input_and_no_elapsed_time() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(0.0);
        let inputs = [(constant::ACTION_POTENTIAL, Weight(1.0))];
        let membrane_potential = neuron.step(elapsed_time, &inputs);
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_with_high_input_and_no_weight() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);
        let inputs = [(constant::ACTION_POTENTIAL, Weight(0.0))];

        let membrane_potential = neuron.step(elapsed_time, &inputs);
        assert!(membrane_potential.is_none());

        let membrane_potential = neuron.step(elapsed_time, &inputs);
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_when_input_is_too_low() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);

        let nearly_threshold = [(
            MembranePotential(constant::THRESHOLD_POTENTIAL.0 - 0.1),
            Weight(1.0),
        )];
        let membrane_potential = neuron.step(elapsed_time, &nearly_threshold);
        assert!(membrane_potential.is_none());

        let no_inputs = [(MembranePotential(0.0), Weight(1.0))];
        let membrane_potential = neuron.step(elapsed_time, &no_inputs);
        assert!(membrane_potential.is_none());
    }
}
