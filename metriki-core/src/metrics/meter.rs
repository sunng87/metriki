use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};

use crossbeam_utils::atomic::AtomicCell;

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

use crate::utils;

/// Meters are used to calculate rate of an event.
#[derive(Debug)]
pub struct Meter {
    moving_avarages: ExponentiallyWeightedMovingAverages,
    count: AtomicU64,
    start_time: SystemTime,
}

impl Meter {
    pub(crate) fn new() -> Meter {
        Meter {
            moving_avarages: ExponentiallyWeightedMovingAverages::new(),
            count: AtomicU64::from(0),
            start_time: SystemTime::now(),
        }
    }

    pub fn mark(&self) {
        self.mark_n(1)
    }

    pub fn mark_n(&self, n: u64) {
        self.count.fetch_add(n, Ordering::Relaxed);
        self.moving_avarages.tick_if_needed();
        self.moving_avarages.update(n);
    }

    pub fn m1_rate(&self) -> f64 {
        self.moving_avarages.tick_if_needed();
        self.moving_avarages.m1_rate()
    }

    pub fn m5_rate(&self) -> f64 {
        self.moving_avarages.tick_if_needed();
        self.moving_avarages.m5_rate()
    }

    pub fn m15_rate(&self) -> f64 {
        self.moving_avarages.tick_if_needed();
        self.moving_avarages.m15_rate()
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn mean_rate(&self) -> f64 {
        let count = self.count();
        if count > 0 {
            if let Ok(elapsed) = SystemTime::now()
                .duration_since(self.start_time)
                .map(|d| d.as_secs() as f64)
            {
                count as f64 / elapsed
            } else {
                0f64
            }
        } else {
            0f64
        }
    }
}

#[derive(Debug)]
struct ExponentiallyWeightedMovingAverage {
    alpha: f64,
    interval_nanos: u64,

    uncounted: AtomicCell<u64>,
    rate: AtomicCell<Option<f64>>,
}

impl ExponentiallyWeightedMovingAverage {
    fn new(alpha: f64, interval_secs: u64) -> ExponentiallyWeightedMovingAverage {
        ExponentiallyWeightedMovingAverage {
            alpha,
            interval_nanos: utils::secs_to_nanos(interval_secs),

            uncounted: AtomicCell::new(0),
            rate: AtomicCell::new(None),
        }
    }

    fn update(&self, n: u64) {
        self.uncounted.fetch_add(n);
    }

    fn tick(&self) {
        let count = self.uncounted.swap(0);
        let instant_rate = count as f64 / self.interval_nanos as f64;

        if let Some(prev_rate) = self.rate.load() {
            let new_rate = prev_rate + (self.alpha * (instant_rate - prev_rate));
            self.rate.store(Some(new_rate));
        } else {
            self.rate.store(Some(instant_rate));
        }
    }

    fn get_rate(&self) -> f64 {
        if let Some(rate) = self.rate.load() {
            rate * utils::secs_to_nanos(1) as f64
        } else {
            0f64
        }
    }
}

#[derive(Debug)]
struct ExponentiallyWeightedMovingAverages {
    m1: ExponentiallyWeightedMovingAverage,
    m5: ExponentiallyWeightedMovingAverage,
    m15: ExponentiallyWeightedMovingAverage,

    last_tick: AtomicCell<Instant>,
}

#[inline]
fn alpha(interval_secs: u64, minutes: u64) -> f64 {
    1.0 - (-(interval_secs as f64) / 60.0 / minutes as f64).exp()
}

const DEFAULT_INTERVAL_SECS: u64 = 5;
const DEFAULT_INTERVAL_MILLIS: u64 = DEFAULT_INTERVAL_SECS * 1000;

impl ExponentiallyWeightedMovingAverages {
    fn new() -> ExponentiallyWeightedMovingAverages {
        ExponentiallyWeightedMovingAverages {
            m1: ExponentiallyWeightedMovingAverage::new(
                alpha(DEFAULT_INTERVAL_SECS, 1),
                DEFAULT_INTERVAL_SECS,
            ),

            m5: ExponentiallyWeightedMovingAverage::new(
                alpha(DEFAULT_INTERVAL_SECS, 5),
                DEFAULT_INTERVAL_SECS,
            ),

            m15: ExponentiallyWeightedMovingAverage::new(
                alpha(DEFAULT_INTERVAL_SECS, 15),
                DEFAULT_INTERVAL_SECS,
            ),

            last_tick: AtomicCell::new(Instant::now()),
        }
    }

    fn update(&self, n: u64) {
        self.m1.update(n);
        self.m5.update(n);
        self.m15.update(n);
    }

    fn tick_if_needed(&self) {
        let previous_tick = self.last_tick.load();
        let current_tick = Instant::now();

        let tick_age = (current_tick - previous_tick).as_millis() as u64;

        if tick_age > DEFAULT_INTERVAL_MILLIS {
            let latest_tick = current_tick
                .checked_sub(Duration::from_millis(tick_age % DEFAULT_INTERVAL_MILLIS))
                .unwrap();

            if self
                .last_tick
                .compare_exchange(previous_tick, latest_tick)
                .is_ok()
            {
                let required_ticks = tick_age / DEFAULT_INTERVAL_MILLIS;
                for _ in 0..required_ticks {
                    self.m1.tick();
                    self.m5.tick();
                    self.m15.tick();
                }
            }
        }
    }

    fn m1_rate(&self) -> f64 {
        self.m1.get_rate()
    }

    fn m5_rate(&self) -> f64 {
        self.m5.get_rate()
    }

    fn m15_rate(&self) -> f64 {
        self.m15.get_rate()
    }
}

#[cfg(feature = "ser")]
impl Serialize for Meter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;

        map.serialize_entry("count", &self.count())?;
        map.serialize_entry("m1_rate", &self.m1_rate())?;
        map.serialize_entry("m5_rate", &self.m5_rate())?;
        map.serialize_entry("m15_rate", &self.m15_rate())?;

        map.end()
    }
}
