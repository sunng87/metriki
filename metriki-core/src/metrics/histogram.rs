use std::sync::{Arc, Mutex};

use exponential_decay_histogram::{ExponentialDecayHistogram, Snapshot};

#[derive(Debug)]
pub struct Histogram {
    inner: Arc<Mutex<ExponentialDecayHistogram>>,
}

pub struct HistogramSnapshot {
    inner: Snapshot,
}

impl Histogram {
    pub(crate) fn new() -> Histogram {
        let inner = ExponentialDecayHistogram::builder().build();

        Histogram {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn update(&self, value: i64) {
        let mut inner = self.inner.lock().unwrap();
        inner.update(value as i64);
    }

    pub fn snapshot(&self) -> HistogramSnapshot {
        let inner = self.inner.lock().unwrap();
        let snapshot = inner.snapshot();

        HistogramSnapshot { inner: snapshot }
    }
}

impl HistogramSnapshot {
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
