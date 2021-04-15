use std::time::Instant;

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

    pub fn start(&self) -> TimerContext {
        self.start_at(Instant::now())
    }

    pub fn start_at(&self, start_at: Instant) -> TimerContext {
        self.rate.mark();
        TimerContext {
            start_at,
            timer: self,
        }
    }

    pub fn rate(&self) -> &Meter {
        &self.rate
    }

    pub fn latency(&self) -> HistogramSnapshot {
        self.latency.snapshot()
    }
}

impl<'a> TimerContext<'a> {
    pub fn stop(&self) {
        let elapsed = Instant::now() - self.start_at;
        let elapsed_ms = elapsed.as_millis();

        self.timer.latency.update(elapsed_ms as i64);
    }
}

impl<'a> Drop for TimerContext<'a> {
    fn drop(&mut self) {
        self.stop()
    }
}
