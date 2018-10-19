use std::error::Error;
use std::fmt::{self, Display};
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    interval: Duration,
    last_execution: Instant,
}

impl FixedIntervalSleeperImpl {
    pub(crate) fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_execution: Instant::now(),
        }
    }
}

impl FixedIntervalSleeper for FixedIntervalSleeperImpl {
    fn sleep_until_interval_passed(&mut self) -> Result<(), FixedIntervalSleeperError> {
        let elapsed = self.last_execution.elapsed();
        self.last_execution += self.interval;

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

    #[test]
    fn sleeps_enough() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);

        let instant = Instant::now();

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_ok());
        assert!(instant.elapsed() >= duration);
    }

    #[test]
    fn does_not_oversleep() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);

        let instant = Instant::now();

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_ok());
        assert!(instant.elapsed() <= duration + Duration::from_millis(10));
    }

    #[test]
    fn is_err_when_too_much_time_has_passed() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);

        sleep(duration * 2);

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_err());
    }

}
