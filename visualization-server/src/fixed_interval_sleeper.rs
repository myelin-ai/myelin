use std::error::Error;
use std::fmt::{self, Display};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub(crate) trait FixedIntervalSleeper {
    fn start(&mut self);
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
    last_execution: Option<Instant>,
}

impl FixedIntervalSleeperImpl {
    pub(crate) fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_execution: None,
        }
    }
}

impl FixedIntervalSleeper for FixedIntervalSleeperImpl {
    fn start(&mut self) {
        self.last_execution = Some(Instant::now());
    }

    fn sleep_until_interval_passed(&mut self) -> Result<(), FixedIntervalSleeperError> {
        let mut last_execution = self.last_execution.expect("start method was not called");
        let elapsed = last_execution.elapsed();
        last_execution += self.interval;

        if elapsed > self.interval {
            return Err(FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(
                elapsed,
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

    #[should_panic]
    #[test]
    fn panics_when_start_was_not_called() {
        let mut sleeper = FixedIntervalSleeperImpl::new(Duration::from_millis(50));
        let _ = sleeper.sleep_until_interval_passed();
    }

    #[test]
    fn sleeps_enough() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);
        sleeper.start();

        let instant = Instant::now();

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_ok());
        assert!(instant.elapsed() >= duration);
    }

    #[test]
    fn does_not_oversleep() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);
        sleeper.start();

        let instant = Instant::now();

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_ok());
        assert!(instant.elapsed() <= duration + Duration::from_millis(10));
    }

    #[test]
    fn is_err_when_too_much_time_has_passed() {
        let duration = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::new(duration);
        sleeper.start();

        sleep(duration * 2);

        let result = sleeper.sleep_until_interval_passed();

        assert!(result.is_err());

        match result.err().unwrap() {
            FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(elapsed_time) => {
                assert!(elapsed_time >= duration * 2);
            }
        }
    }

}
