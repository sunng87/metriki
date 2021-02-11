use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::metrics::{Counter, Histogram, Meter, Metric};

/// Entrypoint of all metrics
///
#[derive(Default, Debug)]
pub struct MetricsRegistry {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Default, Debug)]
struct Inner {
    metrics: HashMap<String, Metric>,
}

impl MetricsRegistry {
    /// Return `Meter` that has been registered or just created and resgitered.
    /// Panic if a metric is already register but is not meter
    pub fn meter(&self, name: &str) -> Arc<Meter> {
        let inner = self.inner.read().unwrap();

        if !inner.metrics.contains_key(name) {
            let mut inner_write = self.inner.write().unwrap();
            let meter = Meter::new();
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Meter(Arc::new(meter)));
        }

        let metric = inner.metrics.get(name).unwrap();
        match metric {
            Metric::Meter(ref m) => m.clone(),
            _ => panic!("A metric with same name and different type is already registered."),
        }
    }

    pub fn histogram(&self, name: &str) -> Arc<Histogram> {
        let inner = self.inner.read().unwrap();

        if !inner.metrics.contains_key(name) {
            let mut inner_write = self.inner.write().unwrap();
            let histogram = Histogram::new();
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Histogram(Arc::new(histogram)));
        }

        let metric = inner.metrics.get(name).unwrap();
        match metric {
            Metric::Histogram(ref m) => m.clone(),
            _ => panic!("A metric with same name and different type is already registered."),
        }
    }

    pub fn counter(&self, name: &str) -> Arc<Counter> {
        let inner = self.inner.read().unwrap();

        if !inner.metrics.contains_key(name) {
            let mut inner_write = self.inner.write().unwrap();
            let counter = Counter::new();
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Counter(Arc::new(counter)));
        }

        let metric = inner.metrics.get(name).unwrap();
        match metric {
            Metric::Counter(ref m) => m.clone(),
            _ => panic!("A metric with same name and different type is already registered."),
        }
    }

    pub fn snapshots(&self) -> HashMap<String, Metric> {
        let inner = self.inner.read().unwrap();
        inner.metrics.clone()
    }
}
