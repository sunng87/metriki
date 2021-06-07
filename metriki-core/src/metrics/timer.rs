use std::time::Instant;

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

use super::{Histogram, HistogramSnapshot, Meter};

/// Timers are combination of `Histogram` and `Meter`.
///
/// Timers are handy for tracking rate and latency of a special part of code.
#[derive(Debug)]
pub struct Timer {
    rate: Meter,
    latency: Histogram,
}

#[derive(Debug)]
pub struct TimerContext<'a> {
    start_at: Instant,
    timer: &'a Timer,
}

impl Timer {
    pub(crate) fn new() -> Timer {
        Timer {
            rate: Meter::new(),
            latency: Histogram::new(),
        }
    }

    /// Start a timer instance for recording.
    /// The returned `TimerContext` can be stopped or dropped to record its timing.
    pub fn start(&self) -> TimerContext {
        self.start_at(Instant::now())
    }

    /// Start a timer instance for recording that started at given time.
    /// The returned `TimerContext` can be stopped or dropped to record its timing.
    pub fn start_at(&self, start_at: Instant) -> TimerContext {
        self.rate.mark();
        TimerContext {
            start_at,
            timer: self,
        }
    }

    /// Execute closure and record its execution with this timer.
    pub fn scoped<F, R>(&self, f: F) -> R
    where
        F: Fn() -> R,
    {
        let ctx = self.start();
        let result = f();
        ctx.stop();
        result
    }

    /// Returns the rates of timer
    pub fn rate(&self) -> &Meter {
        &self.rate
    }

    /// Returns the histogram of latency distribution of this timer
    pub fn latency(&self) -> HistogramSnapshot {
        self.latency.snapshot()
    }
}

impl<'a> TimerContext<'a> {
    pub fn stop(&self) {
        let elapsed = Instant::now() - self.start_at;
        let elapsed_ms = elapsed.as_millis();

        self.timer.latency.update(elapsed_ms as u64);
    }
}

impl<'a> Drop for TimerContext<'a> {
    fn drop(&mut self) {
        self.stop()
    }
}

#[cfg(feature = "ser")]
impl Serialize for Timer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(13))?;

        let rate = self.rate();
        let latency = self.latency();

        map.serialize_entry("count", &rate.count())?;
        map.serialize_entry("m1_rate", &rate.m1_rate())?;
        map.serialize_entry("m5_rate", &rate.m5_rate())?;
        map.serialize_entry("m15_rate", &rate.m15_rate())?;

        map.serialize_entry("mean", &latency.mean())?;
        map.serialize_entry("max", &latency.max())?;
        map.serialize_entry("min", &latency.min())?;
        map.serialize_entry("stddev", &latency.stddev())?;

        map.serialize_entry("p50", &latency.quantile(0.5))?;
        map.serialize_entry("p75", &latency.quantile(0.75))?;
        map.serialize_entry("p90", &latency.quantile(0.9))?;
        map.serialize_entry("p99", &latency.quantile(0.99))?;
        map.serialize_entry("p999", &latency.quantile(0.999))?;

        map.end()
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::Timer;

    #[test]
    fn test_drop_timer_context() {
        let timer = Timer::new();
        // traced block
        let t = timer.start();
        t.stop();

        assert!(timer.rate().count() == 1);

        {
            timer.start();
            std::thread::sleep(Duration::from_millis(500));
        }

        assert!(timer.rate().count() == 2);
    }
}
