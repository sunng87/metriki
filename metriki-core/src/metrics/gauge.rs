use std::fmt;

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

/// Trait for gauge impls
pub trait GaugeProvider: Send + Sync {
    fn value(&self) -> f64;
}

impl GaugeProvider for dyn Fn() -> f64 + Send + Sync {
    fn value(&self) -> f64 {
        self()
    }
}

/// Gauges are used to measure the instantaneous value of something.
pub struct Gauge {
    func: Box<dyn GaugeProvider>,
}

impl Gauge {
    pub(crate) fn new(f: Box<dyn GaugeProvider>) -> Gauge {
        Gauge { func: f }
    }

    pub fn value(&self) -> f64 {
        self.func.value()
    }
}

impl fmt::Debug for Gauge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Gauge").finish()
    }
}

#[cfg(feature = "ser")]
impl Serialize for Gauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}
