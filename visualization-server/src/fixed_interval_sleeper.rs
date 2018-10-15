use std::error::Error;
use std::fmt::{self, Display};
use std::thread::sleep;
use std::time::Duration;

pub(crate) trait Instant {
    fn elapsed(&mut self) -> Duration;
}

type InstantFactoryFn = Box<dyn FnMut() -> Box<dyn Instant>>;

pub struct RealInstant {
    instant: std::time::Instant,
}

impl Instant for RealInstant {
    fn elapsed(&mut self) -> Duration {
        self.instant.elapsed()
    }
}

pub(crate) fn real_instant_factory_fn() -> Box<dyn Instant> {
    Box::new(RealInstant {
        instant: std::time::Instant::now(),
    })
}

pub(crate) trait FixedIntervalSleeper {
    fn sleep_until_interval_passed(&mut self) -> Result<(), FixedIntervalSleeperError>;
}

#[derive(Debug)]
pub(crate) enum FixedIntervalSleeperError {
    /// Returns the duration since the last execution
    ElapsedTimeIsGreaterThanInterval(Duration),
}

impl Error for FixedIntervalSleeperError {}
impl Display for FixedIntervalSleeperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(duration) => {
                write!(f, "Elapsed time greater than interval by {:?}", duration)
            }
        }
    }
}

pub(crate) struct FixedIntervalSleeperImpl {
    instant_factory_fn: InstantFactoryFn,
    interval: Duration,
    last_execution: Box<dyn Instant>,
}

impl FixedIntervalSleeperImpl {
    pub(crate) fn new(mut instant_factory_fn: InstantFactoryFn, interval: Duration) -> Self {
        Self {
            last_execution: instant_factory_fn(),
            instant_factory_fn,
            interval,
        }
    }
}

impl FixedIntervalSleeper for FixedIntervalSleeperImpl {
    fn sleep_until_interval_passed(&mut self) -> Result<(), FixedIntervalSleeperError> {
        let elapsed = self.last_execution.elapsed();

        self.last_execution = (self.instant_factory_fn)();

        if elapsed > self.interval {
            return Err(FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(
                elapsed - self.interval,
            ));
        }

        let delta = self.interval - elapsed;
        sleep(delta);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InstantMock {
        instances: Vec<u64>,
    }

    impl Instant for InstantMock {
        fn elapsed(&mut self) -> Duration {
            Duration::from_secs(
                self.instances
                    .pop()
                    .expect("elapsed was called unexpectedly"),
            )
        }
    }

    pub(crate) fn create_mock_instant_factory_fn_factory(instances: Vec<u64>) -> InstantFactoryFn {
        Box::new(move || {
            let mut instances = instances.clone();
            instances.reverse();

            Box::new(InstantMock { instances })
        })
    }

    #[test]
    fn returns_error_when_elapsed_time_is_greater_than_interval() {
        let instant_factory = create_mock_instant_factory_fn_factory(vec![0, 2]);
        let interval = Duration::from_secs(1);

        let mut fixed_interval_sleeper = FixedIntervalSleeperImpl::new(instant_factory, interval);

        let result = fixed_interval_sleeper.sleep_until_interval_passed();

        assert!(result.is_err());
    }
}
