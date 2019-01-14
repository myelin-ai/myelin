//! [Source](https://books.google.com/books?id=A8H_9S4E0I4C&pg=PA55&lpg=PA55&focus=viewport)

use crate::{MembranePotential, Milliseconds};

pub(crate) const RESTING_POTENTIAL: MembranePotential = MembranePotential(-70.0);
pub(crate) const THRESHOLD_POTENTIAL: MembranePotential = MembranePotential(-55.0);
pub(crate) const ACTION_POTENTIAL: MembranePotential = MembranePotential(40.0);

pub(crate) const PASSIVE_REPOLARIZATION_FACTOR: f64 = 0.01;

pub(crate) const SPIKE_DURATION: Milliseconds = Milliseconds(2.0);
