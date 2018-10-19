use std::error::Error;
use std::fmt::{self, Display};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub(crate) trait FixedIntervalSleeper {
    fn register_work_started(&mut self);
    fn sleep_until_interval_passed(
        &self,
        interval: Duration,
    ) -> Result<(), FixedIntervalSleeperError>;
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

#[derive(Default)]
pub(crate) struct FixedIntervalSleeperImpl {
    instant_that_the_work_was_started: Option<Instant>,
}

impl FixedIntervalSleeper for FixedIntervalSleeperImpl {
    fn register_work_started(&mut self) {
        self.instant_that_the_work_was_started = Some(Instant::now());
    }

    fn sleep_until_interval_passed(
        &self,
        interval: Duration,
    ) -> Result<(), FixedIntervalSleeperError> {
        let mut instant_that_the_work_was_started = self
            .instant_that_the_work_was_started
            .expect("work_started method was not called");
        let elapsed = instant_that_the_work_was_started.elapsed();

        if elapsed > interval {
            return Err(FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(
                elapsed,
            ));
        }

        let delta = interval - elapsed;
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
        let mut sleeper = FixedIntervalSleeperImpl::default();
        let _ = sleeper.sleep_until_interval_passed(Duration::from_millis(50));
    }

    #[test]
    fn sleeps_enough() {
        let interval = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::default();

        let instant = Instant::now();

        sleeper.register_work_started();
        let result = sleeper.sleep_until_interval_passed(interval);

        assert!(result.is_ok());
        assert!(instant.elapsed() >= interval);
    }

    #[test]
    fn does_not_oversleep() {
        let interval = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::default();

        let instant = Instant::now();

        sleeper.register_work_started();
        let result = sleeper.sleep_until_interval_passed(interval);

        assert!(result.is_ok());
        assert!(instant.elapsed() <= interval + Duration::from_millis(10));
    }

    #[test]
    fn is_err_when_too_much_time_has_passed() {
        let interval = Duration::from_millis(50);
        let mut sleeper = FixedIntervalSleeperImpl::default();

        sleeper.register_work_started();

        sleep(interval * 2);

        let result = sleeper.sleep_until_interval_passed(interval);

        assert!(result.is_err());

        match result.err().unwrap() {
            FixedIntervalSleeperError::ElapsedTimeIsGreaterThanInterval(elapsed_time) => {
                assert!(elapsed_time >= interval * 2);
            }
        }
    }

}
