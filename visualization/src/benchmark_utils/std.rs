use std::{fmt, time};

pub(crate) fn print<D>(value: D)
where
    D: fmt::Display,
{
    println!("{}", value)
}

#[derive(Debug)]
pub(crate) struct Instant {
    inner: time::Instant,
}

impl Instant {
    pub(crate) fn now() -> Self {
        Instant {
            inner: time::Instant::now(),
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        Duration {
            inner: self.inner.elapsed(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Duration {
    inner: time::Duration,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}µs", self.inner.as_micros())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Durations {
    inner: Vec<Duration>,
}

impl Durations {
    pub(crate) fn add_duration(&mut self, duration: Duration) {
        self.inner.push(duration);
    }

    pub(crate) fn average(&self) -> Duration {
        Duration {
            inner: self
                .inner
                .iter()
                .fold(time::Duration::new(0, 0), |acc, value| acc + value.inner)
                / self.inner.len() as u32,
        }
    }

    pub(crate) fn count(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn clear(&mut self) {
        self.inner.clear();
    }
}
