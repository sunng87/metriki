use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use exponential_decay_histogram::{ExponentialDecayHistogram, Snapshot};

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
