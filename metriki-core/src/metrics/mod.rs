use std::sync::Arc;

#[cfg(feature = "ser")]
use serde::ser::SerializeSeq;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

mod counter;
mod gauge;
mod histogram;
mod meter;
mod timer;

#[derive(Clone, Debug)]
pub enum Metric {
    Meter(Arc<Meter>),
    Timer(Arc<Timer>),
    Gauge(Arc<Gauge>),
    Histogram(Arc<Histogram>),
    Counter(Arc<Counter>),
}

impl Metric {
    /// Create default meter
    pub fn meter() -> Meter {
        Meter::new()
    }

    /// Create default timer
    pub fn timer() -> Timer {
        Timer::new()
    }

    /// Create gauge with given function
    pub fn gauge(f: GaugeFn) -> Gauge {
        Gauge::new(f)
    }

    /// Create default histogram
    pub fn histogram() -> Histogram {
        Histogram::new()
    }

    /// Create default counter
    pub fn counter() -> Counter {
        Counter::new()
    }
}

impl From<Meter> for Metric {
    fn from(f: Meter) -> Metric {
        Metric::Meter(Arc::new(f))
    }
}

impl From<Timer> for Metric {
    fn from(f: Timer) -> Metric {
        Metric::Timer(Arc::new(f))
    }
}

impl From<Counter> for Metric {
    fn from(f: Counter) -> Metric {
        Metric::Counter(Arc::new(f))
    }
}

impl From<Gauge> for Metric {
    fn from(f: Gauge) -> Metric {
        Metric::Gauge(Arc::new(f))
    }
}

impl From<Histogram> for Metric {
    fn from(f: Histogram) -> Metric {
        Metric::Histogram(Arc::new(f))
    }
}

#[cfg(feature = "ser")]
impl Serialize for Metric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Metric::Meter(inner) => inner.serialize(serializer),
            Metric::Timer(inner) => inner.serialize(serializer),
            Metric::Gauge(inner) => inner.serialize(serializer),
            Metric::Histogram(inner) => inner.serialize(serializer),
            Metric::Counter(inner) => inner.serialize(serializer),
        }
    }
}

pub use counter::Counter;
pub use gauge::{Gauge, GaugeFn};
pub use histogram::{Histogram, HistogramSnapshot};
pub use meter::Meter;
pub use timer::Timer;
