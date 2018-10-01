use std::fmt;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::window;

pub(crate) fn print<D>(value: D)
where
    D: fmt::Display,
{
    console::log_1(&JsValue::from(&format!("{}", value)));
}

#[derive(Debug)]
pub(crate) struct Instant {
    value: f64,
}

impl Instant {
    pub(crate) fn now() -> Self {
        Instant {
            value: performance_now(),
        }
    }

    pub(crate) fn elapsed(&self) -> Duration {
        Duration {
            value: performance_now() - self.value,
        }
    }
}

fn performance_now() -> f64 {
    window().unwrap().performance().unwrap().now()
}

#[derive(Debug)]
pub(crate) struct Duration {
    value: f64,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.value)
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
            value: self.inner.iter().map(|d| d.value).sum::<f64>() / self.inner.len() as f64,
        }
    }

    pub(crate) fn count(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn clear(&mut self) {
        self.inner.clear();
    }
}
