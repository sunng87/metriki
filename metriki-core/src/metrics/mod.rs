use std::sync::Arc;

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
    pub fn meter() -> Arc<Meter> {
        Meter::new().into()
    }

    /// Create default timer
    pub fn timer() -> Arc<Timer> {
        Timer::new().into()
    }

    /// Create gauge with given function
    pub fn gauge(f: GaugeFn) -> Arc<Gauge> {
        Gauge::new(f).into()
    }

    /// Create default histogram
    pub fn histogram() -> Arc<Histogram> {
        Histogram::new().into()
    }

    /// Create default counter
    pub fn counter() -> Arc<Counter> {
        Counter::new().into()
    }
}

impl From<Arc<Meter>> for Metric {
    fn from(f: Arc<Meter>) -> Metric {
        Metric::Meter(f)
    }
}

impl From<Arc<Timer>> for Metric {
    fn from(f: Arc<Timer>) -> Metric {
        Metric::Timer(f)
    }
}

impl From<Arc<Counter>> for Metric {
    fn from(f: Arc<Counter>) -> Metric {
        Metric::Counter(f)
    }
}

impl From<Arc<Gauge>> for Metric {
    fn from(f: Arc<Gauge>) -> Metric {
        Metric::Gauge(f)
    }
}

impl From<Arc<Histogram>> for Metric {
    fn from(f: Arc<Histogram>) -> Metric {
        Metric::Histogram(f)
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
