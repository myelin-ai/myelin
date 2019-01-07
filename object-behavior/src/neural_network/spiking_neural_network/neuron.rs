use super::constant;
use crate::neural_network::*;
use std::f64::consts::E;

/// A spiking neuron
#[derive(Debug)]
pub struct SpikingNeuron {
    current_membrane_potential: MembranePotential,
    current_threshold: MembranePotential,
    current_phase: Phase,
    elapsed_time_in_current_phase: Milliseconds,
}

impl SpikingNeuron {
    /// Constructs a new neuron
    pub fn new() -> Self {
        Self {
            current_threshold: constant::THRESHOLD_POTENTIAL,
            current_membrane_potential: constant::RESTING_POTENTIAL,
            current_phase: Phase::RestingState,
            elapsed_time_in_current_phase: Milliseconds(0.0),
        }
    }

    /// Update the state of the neuron and return it
    pub fn step(
        &mut self,
        time_since_last_step: Milliseconds,
        inputs: &[(MembranePotential, Weight)],
    ) -> Option<MembranePotential> {
        self.update_phase(time_since_last_step);
        self.handle_phase(inputs, time_since_last_step);
        self.output()
    }

    fn update_phase(&mut self, time_since_last_step: Milliseconds) {
        self.elapsed_time_in_current_phase.0 += time_since_last_step.0;
        match &self.current_phase {
            Phase::RestingState => {
                if self.current_membrane_potential > self.current_threshold {
                    self.current_phase = Phase::Depolarization;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
            Phase::Depolarization => {
                if self.current_membrane_potential >= constant::ACTION_POTENTIAL {
                    self.current_membrane_potential = constant::ACTION_POTENTIAL;
                    self.current_phase = Phase::ActionPotential;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
            Phase::ActionPotential => {
                self.current_phase = Phase::Repolarization;
                self.elapsed_time_in_current_phase = Default::default();
            }
            Phase::Repolarization => {
                if self.current_membrane_potential <= constant::RESTING_POTENTIAL {
                    self.current_phase = Phase::RefractoryPeriod;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
            Phase::RefractoryPeriod => {
                if self.current_membrane_potential > constant::RESTING_POTENTIAL {
                    self.current_phase = Phase::RestingState;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
        }
    }

    fn handle_phase(
        &mut self,
        inputs: &[(MembranePotential, Weight)],
        time_since_last_step: Milliseconds,
    ) {
        match &self.current_phase {
            Phase::RestingState => self.handle_resting_state(inputs, time_since_last_step),
            Phase::Depolarization => self.handle_depolarization(inputs),
            Phase::ActionPotential => self.handle_action_potential(inputs),
            Phase::Repolarization => self.handle_repolarization(inputs),
            Phase::RefractoryPeriod => self.handle_refractory_period(inputs),
        }
    }

    fn handle_resting_state(
        &mut self,
        inputs: &[(MembranePotential, Weight)],
        time_since_last_step: Milliseconds,
    ) {
        self.current_membrane_potential.0 +=
            passive_repolarization(self.current_membrane_potential, time_since_last_step)
                + sum_inputs(inputs).0;
    }

    fn handle_depolarization(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential.0 =
            constant::RESTING_POTENTIAL.0 + spike(self.elapsed_time_in_current_phase);
    }

    fn handle_action_potential(&mut self, _inputs: &[(MembranePotential, Weight)]) {}

    fn handle_repolarization(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential.0 -= spike(self.elapsed_time_in_current_phase)
    }

    fn handle_refractory_period(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential.0 = constant::RESTING_POTENTIAL.0
            + 3.0 * self.elapsed_time_in_current_phase.0.powf(2.0)
            - 3.0
    }

    fn is_above_threshold(&self) -> bool {
        self.current_membrane_potential >= self.current_threshold
    }

    fn output(&self) -> Option<MembranePotential> {
        if self.is_above_threshold() {
            Some(MembranePotential(
                self.current_membrane_potential.0 - constant::THRESHOLD_POTENTIAL.0,
            ))
        } else {
            None
        }
    }
}

impl Default for SpikingNeuron {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum Phase {
    RestingState,
    Depolarization,
    ActionPotential,
    Repolarization,
    RefractoryPeriod,
}

fn passive_repolarization(
    current_membrane_potential: MembranePotential,
    time_since_last_step: Milliseconds,
) -> f64 {
    let delta = constant::RESTING_POTENTIAL.0 - current_membrane_potential.0;
    delta * constant::PASSIVE_REPOLARIZATION_FACTOR * time_since_last_step.0
}

fn spike(elapsed_ms: Milliseconds) -> f64 {
    let shifted_input = elapsed_ms.0 + 3.0;
    shifted_input * (E / 1.1).powf(shifted_input)
}

fn sum_inputs(inputs: &[(MembranePotential, Weight)]) -> MembranePotential {
    MembranePotential(
        inputs
            .iter()
            .map(|(membrane_potential, weight)| {
                let input_shifted_into_positive_range =
                    membrane_potential.0 - constant::RESTING_POTENTIAL.0;
                input_shifted_into_positive_range * weight.0
            })
            .sum(),
    )
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
    fn emits_no_potential_with_high_input_and_no_weight() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);
        let inputs = [(constant::ACTION_POTENTIAL, Weight(0.0))];

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
    }

    #[test]
    fn spikes_with_extremely_high_input() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);

        let inputs = [(MembranePotential(1000.0), Weight(1000.0))];

        let membrane_potential = neuron.step(elapsed_time, &inputs);
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn spikes_with_input_of_threshold() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);

        let inputs = [(constant::THRESHOLD_POTENTIAL, Weight(1.0))];

        let membrane_potential = neuron.step(elapsed_time, &inputs);
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn spike_goes_away_after_a_while() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = Milliseconds(10.0);

        let inputs = [(constant::THRESHOLD_POTENTIAL, Weight(1.0))];
        neuron.step(elapsed_time, &inputs);

        for _ in 0..100 {
            neuron.step(elapsed_time, &[]);
        }
        let membrane_potential = neuron.step(elapsed_time, &[]);
        assert!(membrane_potential.is_none());
    }
}
