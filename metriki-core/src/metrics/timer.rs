use std::time::Instant;

use super::{Histogram, HistogramSnapshot, Meter};

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
    pub fn new() -> Timer {
        Timer {
            rate: Meter::new(),
            latency: Histogram::new(),
        }
    }

    pub fn start(&self) -> TimerContext {
        self.rate.mark();
        TimerContext {
            start_at: Instant::now(),
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
    pub fn stop(self) {
        let elapsed = Instant::now() - self.start_at;
        let elapsed_ms = elapsed.as_millis();

        self.timer.latency.update(elapsed_ms as i64);
    }
}
