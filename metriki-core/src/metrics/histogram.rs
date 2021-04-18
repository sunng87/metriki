use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use exponential_decay_histogram::{ExponentialDecayHistogram, Snapshot};

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

/// Histograms are used to record the distribution of data over time.
///
/// By default, `Histogram` uses exponential decay algorithm to avoid
/// record too much data in memory.
#[derive(Debug)]
pub struct Histogram {
    inner: Arc<Mutex<ExponentialDecayHistogram>>,
    count: AtomicU64,
}

pub struct HistogramSnapshot {
    inner: Snapshot,
    count: u64,
}

impl Histogram {
    pub(crate) fn new() -> Histogram {
        let inner = ExponentialDecayHistogram::builder().build();

        Histogram {
            inner: Arc::new(Mutex::new(inner)),
            count: AtomicU64::new(0),
        }
    }

    pub fn update(&self, value: i64) {
        let mut inner = self.inner.lock().unwrap();
        inner.update(value as i64);
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> HistogramSnapshot {
        let inner = self.inner.lock().unwrap();
        let snapshot = inner.snapshot();
        let count = self.count.load(Ordering::Relaxed);

        HistogramSnapshot {
            inner: snapshot,
            count,
        }
    }
}

impl HistogramSnapshot {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn mean(&self) -> f64 {
        self.inner.mean()
    }

    pub fn max(&self) -> i64 {
        self.inner.max()
    }

    pub fn min(&self) -> i64 {
        self.inner.min()
    }

    pub fn stddev(&self) -> f64 {
        self.inner.stddev()
    }

    pub fn quantile(&self, quantile: f64) -> i64 {
        self.inner.value(quantile)
    }
}

#[cfg(feature = "ser")]
impl Serialize for Histogram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(9))?;
        let snapshot = self.snapshot();

        map.serialize_entry("mean", &snapshot.mean())?;
        map.serialize_entry("max", &snapshot.max())?;
        map.serialize_entry("min", &snapshot.min())?;
        map.serialize_entry("stddev", &snapshot.stddev())?;

        map.serialize_entry("p50", &snapshot.quantile(0.5))?;
        map.serialize_entry("p75", &snapshot.quantile(0.75))?;
        map.serialize_entry("p90", &snapshot.quantile(0.9))?;
        map.serialize_entry("p99", &snapshot.quantile(0.99))?;
        map.serialize_entry("p999", &snapshot.quantile(0.999))?;

        map.end()
    }
}
