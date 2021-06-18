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

    /// Convert the Metric to `Meter`
    pub fn as_meter(&self) -> Option<Arc<Meter>> {
        match self {
            Metric::Meter(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Convert the Metric to `Timer`
    pub fn as_timer(&self) -> Option<Arc<Timer>> {
        match self {
            Metric::Timer(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Convert the Metric to `Gauge`
    pub fn as_gauge(&self) -> Option<Arc<Gauge>> {
        match self {
            Metric::Gauge(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Convert the Metric to `Histogram`
    pub fn as_histogram(&self) -> Option<Arc<Histogram>> {
        match self {
            Metric::Histogram(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Convert the Metric to `Counter`
    pub fn as_counter(&self) -> Option<Arc<Counter>> {
        match self {
            Metric::Counter(m) => Some(m.clone()),
            _ => None,
        }
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
pub use timer::{Timer, TimerContext, TimerContext2};
