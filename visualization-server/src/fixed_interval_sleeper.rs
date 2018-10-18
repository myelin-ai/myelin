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

        self.last_execution = Instant::now();

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
