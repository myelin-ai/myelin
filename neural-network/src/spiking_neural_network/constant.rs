//! [Source for potential values](https://books.google.com/books?id=A8H_9S4E0I4C&pg=PA55&lpg=PA55&focus=viewport)
//! [Source for time values](http://www.humanneurophysiology.com/membranepotentials.htm)

use crate::{MembranePotential, Milliseconds};

pub(crate) const RESTING_POTENTIAL: MembranePotential = MembranePotential(-70.0);
pub(crate) const THRESHOLD_POTENTIAL: MembranePotential = MembranePotential(-55.0);
pub(crate) const ACTION_POTENTIAL: MembranePotential = MembranePotential(40.0);

pub(crate) const PASSIVE_REPOLARIZATION_FACTOR: f64 = 0.01;

#[cfg(test)]
pub(crate) const DEPOLARIZATION_DURATION: Milliseconds = Milliseconds(0.7);

#[cfg(test)]
pub(crate) const ACTION_POTENTIAL_DURATION: Milliseconds = Milliseconds(0.1);

#[cfg(test)]
pub(crate) const REPOLARIZATION_DURATION: Milliseconds = Milliseconds(0.7);

pub(crate) const _REFRACTORY_PERIOD_DURATION: Milliseconds = Milliseconds(3.5);

#[cfg(test)]
pub(crate) const SPIKE_DURATION: Milliseconds = Milliseconds(
    DEPOLARIZATION_DURATION.0 + ACTION_POTENTIAL_DURATION.0 + REPOLARIZATION_DURATION.0,
);
