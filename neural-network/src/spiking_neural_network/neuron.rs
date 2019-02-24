use super::constant;
use crate::*;
use std::f64::consts::E;

/// A spiking neuron
#[derive(Debug, Clone)]
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
            elapsed_time_in_current_phase: 0.0,
        }
    }

    /// Update the internal state of the neuron
    ///
    /// # Arguments
    /// - `time_since_last_step`: The time in milliseconds that passed since this method was last called
    /// - `inputs`: The membrane potentials of other connected neurons that influence this neuron right now.
    /// It is assumed that these inputs occure after `time_since_last_step` has passed.
    pub fn step(
        &mut self,
        time_since_last_step: Milliseconds,
        inputs: &[(MembranePotential, Weight)],
    ) {
        self.update_phase(time_since_last_step);
        self.handle_phase(inputs, time_since_last_step);
    }

    /// Returns the neuron's current membrane potential if it is above its current threshold
    /// Call [`step`] to update this value
    ///
    /// [`step`]: #tySpikingNeuron.step
    pub fn membrane_potential(&self) -> Option<MembranePotential> {
        if self.is_above_threshold() {
            Some(self.current_membrane_potential)
        } else {
            None
        }
    }

    /// The current threshold
    pub fn threshold(&self) -> MembranePotential {
        self.current_threshold
    }

    /// The current action potential
    pub fn action_potential(&self) -> MembranePotential {
        constant::ACTION_POTENTIAL
    }

    fn update_phase(&mut self, time_since_last_step: Milliseconds) {
        self.elapsed_time_in_current_phase += time_since_last_step;
        match &self.current_phase {
            Phase::RestingState => {
                if self.current_membrane_potential >= self.current_threshold {
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
                if self.current_membrane_potential <= self.current_threshold {
                    self.current_phase = Phase::Hyperpolarization;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
            Phase::Hyperpolarization => {
                if self.current_membrane_potential <= constant::RESTING_POTENTIAL {
                    self.current_phase = Phase::RefractoryPeriod;
                    self.elapsed_time_in_current_phase = Default::default();
                }
            }
            Phase::RefractoryPeriod => {
                if self.current_membrane_potential >= constant::RESTING_POTENTIAL {
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
            Phase::Hyperpolarization => self.handle_hyperpolarization(inputs),
            Phase::RefractoryPeriod => self.handle_refractory_period(inputs),
        }
    }

    fn handle_resting_state(
        &mut self,
        inputs: &[(MembranePotential, Weight)],
        time_since_last_step: Milliseconds,
    ) {
        self.current_membrane_potential +=
            passive_repolarization(self.current_membrane_potential, time_since_last_step)
                + sum_inputs(inputs);
    }

    fn handle_depolarization(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential =
            constant::RESTING_POTENTIAL + spike(self.elapsed_time_in_current_phase);
    }

    fn handle_action_potential(&mut self, _inputs: &[(MembranePotential, Weight)]) {}

    fn handle_repolarization(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential -= spike(self.elapsed_time_in_current_phase)
    }

    fn handle_refractory_period(&mut self, _inputs: &[(MembranePotential, Weight)]) {
        self.current_membrane_potential =
            constant::RESTING_POTENTIAL + 3.0 * self.elapsed_time_in_current_phase.powf(2.0) - 3.0
    }

    fn handle_hyperpolarization(&mut self, inputs: &[(MembranePotential, Weight)]) {
        self.handle_repolarization(inputs);
        self.current_membrane_potential += sum_inputs(inputs);;
    }

    fn is_above_threshold(&self) -> bool {
        self.current_membrane_potential >= constant::THRESHOLD_POTENTIAL
    }
}

impl Default for SpikingNeuron {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Phase {
    RestingState,
    Depolarization,
    ActionPotential,
    Repolarization,
    Hyperpolarization,
    RefractoryPeriod,
}

fn passive_repolarization(
    current_membrane_potential: MembranePotential,
    time_since_last_step: Milliseconds,
) -> MembranePotential {
    let delta = constant::RESTING_POTENTIAL - current_membrane_potential;
    let repolarization_for_elapsed_time =
        constant::PASSIVE_REPOLARIZATION_FACTOR * time_since_last_step;
    repolarization_for_elapsed_time * delta
}

fn spike(elapsed_ms: Milliseconds) -> f64 {
    let shifted_input = elapsed_ms + 3.0;
    shifted_input * (E / 1.1).powf(shifted_input)
}

fn sum_inputs(inputs: &[(MembranePotential, Weight)]) -> MembranePotential {
    inputs
        .iter()
        .map(|(membrane_potential, weight)| {
            let input_shifted_into_positive_range =
                membrane_potential - constant::RESTING_POTENTIAL;
            input_shifted_into_positive_range * weight
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_constructed() {
        let _neuron = SpikingNeuron::default();
    }

    #[test]
    fn membrane_potential_is_none_when_no_step_is_called() {
        let neuron = SpikingNeuron::default();
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_without_inputs() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 1.0;
        neuron.step(elapsed_time, &[]);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_with_high_input_and_no_weight() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 10.0;
        let inputs = [(constant::ACTION_POTENTIAL, 0.0)];

        neuron.step(elapsed_time, &inputs);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn emits_no_potential_when_input_is_too_low() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 10.0;

        const ARBITRARY_THRESHOLD_DECREMENT: MembranePotential = 0.1;
        let nearly_threshold = [(
            constant::THRESHOLD_POTENTIAL - ARBITRARY_THRESHOLD_DECREMENT,
            1.0,
        )];
        neuron.step(elapsed_time, &nearly_threshold);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn spikes_with_extremely_high_input() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 10.0;

        let inputs = [(1000.0, 1000.0)];

        neuron.step(elapsed_time, &inputs);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_some());
    }

    #[test]
    fn spikes_with_input_of_threshold() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 0.001;

        let inputs = [(constant::THRESHOLD_POTENTIAL, 1.0)];

        neuron.step(elapsed_time, &inputs);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_some());
    }

    #[test]
    fn spikes_with_input_of_threshold_when_factoring_in_weight() {
        let mut neuron = SpikingNeuron::default();
        let elapsed_time = 0.001;

        let inputs = [(constant::THRESHOLD_POTENTIAL / 2.0, 2.0)];

        neuron.step(elapsed_time, &inputs);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_some());
    }

    #[test]
    fn spike_ends_after_many_small_time_steps() {
        let mut neuron = SpikingNeuron::default();
        const SMALL_TIMESTEP: Milliseconds = 0.001;
        let steps = f64::ceil(constant::SPIKE_DURATION / SMALL_TIMESTEP) as u32;

        let inputs = [(constant::THRESHOLD_POTENTIAL, 1.0)];
        neuron.step(SMALL_TIMESTEP, &inputs);

        for _ in 0..steps {
            neuron.step(SMALL_TIMESTEP, &[]);
        }
        neuron.step(SMALL_TIMESTEP, &[]);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn spike_ends_after_many_small_time_steps_when_under_constant_input() {
        let mut neuron = SpikingNeuron::default();
        const SMALL_TIMESTEP: Milliseconds = 0.001;
        let steps = f64::ceil(constant::SPIKE_DURATION / SMALL_TIMESTEP) as u32;

        let inputs = [(constant::THRESHOLD_POTENTIAL, 1.0)];
        neuron.step(SMALL_TIMESTEP, &inputs);

        for _ in 0..steps {
            neuron.step(SMALL_TIMESTEP, &inputs);
        }
        neuron.step(SMALL_TIMESTEP, &[]);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    /// No idea how to best fix this.
    /// The issue is that the state can only transision between one state and another state at a time.
    /// It cannot skip states. Our best efforts in implementating that feature resulted in a recalculation mess.
    /// Not every phase has a constant maximum. Depolarization could be faster when starting at a higher threshold.
    /// Calculating the effectively skipped time is not straightforward.
    #[ignore]
    #[test]
    fn spike_ends_after_one_big_time_step() {
        let mut neuron = SpikingNeuron::default();
        let inputs = [(constant::THRESHOLD_POTENTIAL, 1.0)];
        neuron.step(10.0, &inputs);
        neuron.step(1_000.0, &[]);
        let membrane_potential = neuron.membrane_potential();
        assert!(membrane_potential.is_none());
    }

    #[test]
    fn spike_happens_at_the_right_time_when_using_small_timesteps() {
        let mut neuron = SpikingNeuron::default();
        let inputs = [(constant::THRESHOLD_POTENTIAL, 1.0)];

        const SMALL_TIME_STEP: Milliseconds = 0.01;
        neuron.step(SMALL_TIME_STEP, &inputs);

        const TIME_AFTER_WHICH_A_SPIKE_SHOULD_HAVE_OCCURED: Milliseconds = 1.5;
        let updates_needed_to_reach_spike =
            f64::ceil(TIME_AFTER_WHICH_A_SPIKE_SHOULD_HAVE_OCCURED / SMALL_TIME_STEP) as u32;
        let states: Vec<_> = (0..updates_needed_to_reach_spike)
            .map(|_| {
                neuron.step(SMALL_TIME_STEP, &[]);
                neuron.membrane_potential()
            })
            .filter_map(|state| state)
            .collect();

        let is_any_update_a_spike = states.iter().any(|&state| {
            const MARGIN_OF_ERROR: f64 = 1.0;
            state >= constant::ACTION_POTENTIAL - MARGIN_OF_ERROR
        });
        let hightest_state = states.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(
            is_any_update_a_spike,
            "No state in the specified time reached spike, highest state was {:?}, expected {}",
            hightest_state,
            constant::ACTION_POTENTIAL
        );
    }
}
