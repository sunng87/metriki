use std::fmt;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

/// Gauge value source that returns `f64`.
pub trait GaugeFn: Send + Sync {
    fn value(&self) -> f64;
}

impl<F: Fn() -> f64 + Send + Sync> GaugeFn for F {
    fn value(&self) -> f64 {
        self()
    }
}

/// Gauges are used to measure the instantaneous value of something.
pub struct Gauge {
    func: Box<dyn GaugeFn>,
}

impl Gauge {
    pub(crate) fn new(f: Box<dyn GaugeFn>) -> Gauge {
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

struct Cache<V> {
    expiry: Instant,
    value: V,
}

impl<V> Cache<V> {
    fn expired(&self) -> bool {
        self.expiry < Instant::now()
    }

    fn value(&self) -> &V {
        &self.value
    }
}

/// Gauge implementation that caches the result for given ttl.
///
/// This is designed for gauge functions that are expensive to call.
///
/// ```
/// # use std::time::Duration;
/// # use metriki_core::global::global_registry;
/// # use metriki_core::metrics::CachedGauge;
///
/// global_registry().gauge("gauge_name", CachedGauge::boxed(Box::new(||
///     // gauge function that returns a value
///     42f64
/// ), Duration::from_secs(60)));
/// ```
pub struct CachedGauge {
    func: Box<dyn GaugeFn>,
    cache: Mutex<Option<Cache<f64>>>,
    ttl: Duration,
}

impl CachedGauge {
    /// Create `CachedGauge` with gauge function and given ttl.
    pub fn boxed(func: Box<dyn GaugeFn>, ttl: Duration) -> Box<CachedGauge> {
        Box::new(CachedGauge {
            func,
            ttl,
            cache: Mutex::new(None),
        })
    }
}

impl GaugeFn for CachedGauge {
    fn value(&self) -> f64 {
        let mut cache = self.cache.lock().unwrap();

        if let Some(ref cache_inner) = *cache {
            if !cache_inner.expired() {
                return *cache_inner.value();
            }
        }

        let value = self.func.value();
        let new_cache = Cache {
            expiry: Instant::now() + self.ttl,
            value,
        };

        *cache = Some(new_cache);

        value
    }
}

/// A Gauge that holds a constant value
pub struct StaticGauge(pub f64);

impl GaugeFn for StaticGauge {
    fn value(&self) -> f64 {
        self.0
    }
}
