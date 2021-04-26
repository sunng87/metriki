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
