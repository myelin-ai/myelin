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
    fn to_inner(&self) -> Instant;
}

/// A simple wrapper around an [`Instant`],
/// passing functions to it with no additional functionality.
#[derive(Debug)]
pub struct InstantWrapperImpl {
    instant: Instant,
}

impl InstantWrapperImpl {
    /// Constructs a new [`InstantWrapperImpl`] from an [`Instant`]
    ///
    /// [`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    pub fn new(instant: Instant) -> Self {
        Self { instant }
    }
}

impl InstantWrapper for InstantWrapperImpl {
    fn duration_since(&self, earlier: &dyn InstantWrapper) -> Duration {
        self.instant.duration_since(earlier.to_inner())
    }

    fn elapsed(&self) -> Duration {
        self.instant.elapsed()
    }

    fn to_inner(&self) -> Instant {
        self.instant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    const MAX_DURATION: Duration = Duration::from_millis(20);

    #[test]
    fn returns_inner() {
        let instant = Instant::now();
        let wrapper = InstantWrapperImpl::new(instant);
        assert_eq!(instant, wrapper.to_inner());
    }

    #[test]
    fn duration_since_another_instant_wrapper_is_within_range() {
        let early_wrapper = InstantWrapperImpl::new(Instant::now());

        let sleep_duration = Duration::from_millis(15);
        sleep(sleep_duration);

        let late_wrapper = InstantWrapperImpl::new(Instant::now());

        let elapsed_time = late_wrapper.duration_since(&early_wrapper);
        assert!(elapsed_time >= sleep_duration && elapsed_time < MAX_DURATION);
    }

    #[test]
    fn duration_since_another_instant_wrapper_is_within_range_after_second_sleep() {
        let early_wrapper = InstantWrapperImpl::new(Instant::now());

        let sleep_duration = Duration::from_millis(15);
        sleep(sleep_duration);

        let late_wrapper = InstantWrapperImpl::new(Instant::now());

        sleep(MAX_DURATION);

        let elapsed_time = late_wrapper.duration_since(&early_wrapper);
        assert!(elapsed_time >= sleep_duration && elapsed_time < MAX_DURATION);
    }

    #[test]
    fn elapsed_time_is_within_range() {
        let wrapper = InstantWrapperImpl::new(Instant::now());

        let sleep_duration = Duration::from_millis(15);
        sleep(sleep_duration);

        let elapsed_time = wrapper.elapsed();
        assert!(elapsed_time >= sleep_duration && elapsed_time < MAX_DURATION);
    }
}
