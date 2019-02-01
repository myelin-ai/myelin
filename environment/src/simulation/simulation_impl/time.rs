//! Functionality for dealing with time measurement

#[cfg(any(test, feature = "use-mocks"))]
use mockiato::mockable;
use std::fmt::Debug;
use std::time::{Duration, Instant};

/// Wrapper for [`Instant`], used to mock elapsed time
///
/// [`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
#[cfg_attr(any(test, feature = "use-mocks"), mockable)]
pub trait InstantWrapper: Debug {
    /// Returns the amount of time elapsed from another instant to this one.
    /// # Panics
    /// This function will panic if `earlier` is later than `self`.
    fn duration_since(&self, earlier: &dyn InstantWrapper) -> Duration;

    /// Returns the amount of time elapsed since this instant was created.
    fn elapsed(&self) -> Duration;

    /// Retrieve the wrapped [`Instant`]
    ///
    /// [`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    fn to_instant(&self) -> Instant;
}
