use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::metrics::*;

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
    pub fn new() -> MetricsRegistry {
        MetricsRegistry::default()
    }

    /// Return `Meter` that has been registered or just created and resgitered.
    /// Panic if a metric is already register but is not meter
    pub fn meter(&self, name: &str) -> Arc<Meter> {
        let meter = {
            let inner = self.inner.read().unwrap();

            inner.metrics.get(name).map(|metric| match metric {
                Metric::Meter(ref m) => m.clone(),
                _ => panic!("A metric with same name and different type is already registered."),
            })
        };

        if let Some(m) = meter {
            m
        } else {
            let mut inner_write = self.inner.write().unwrap();
            let meter = Arc::new(Meter::new());
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Meter(meter.clone()));
            meter
        }
    }

    pub fn histogram(&self, name: &str) -> Arc<Histogram> {
        let histo = {
            let inner = self.inner.read().unwrap();

            inner.metrics.get(name).map(|metric| match metric {
                Metric::Histogram(ref m) => m.clone(),
                _ => panic!("A metric with same name and different type is already registered."),
            })
        };

        if let Some(m) = histo {
            m
        } else {
            let mut inner_write = self.inner.write().unwrap();
            let histo = Arc::new(Histogram::new());
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Histogram(histo.clone()));
            histo
        }
    }

    pub fn counter(&self, name: &str) -> Arc<Counter> {
        let counter = {
            let inner = self.inner.read().unwrap();

            inner.metrics.get(name).map(|metric| match metric {
                Metric::Counter(ref m) => m.clone(),
                _ => panic!("A metric with same name and different type is already registered."),
            })
        };

        if let Some(m) = counter {
            m
        } else {
            let mut inner_write = self.inner.write().unwrap();
            let counter = Arc::new(Counter::new());
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Counter(counter.clone()));
            counter
        }
    }

    pub fn timer(&self, name: &str) -> Arc<Timer> {
        let timer = {
            let inner = self.inner.read().unwrap();

            inner.metrics.get(name).map(|metric| match metric {
                Metric::Timer(ref m) => m.clone(),
                _ => panic!("A metric with same name and different type is already registered."),
            })
        };

        if let Some(m) = timer {
            m
        } else {
            let mut inner_write = self.inner.write().unwrap();
            let timer = Arc::new(Timer::new());
            inner_write
                .metrics
                .insert(name.to_owned(), Metric::Timer(timer.clone()));
            timer
        }
    }

    pub fn gauge(&self, name: &str, func: GaugeFn) {
        let mut inner = self.inner.write().unwrap();
        inner
            .metrics
            .insert(name.to_owned(), Metric::Gauge(Arc::new(Gauge::new(func))));
    }

    pub fn snapshots(&self) -> HashMap<String, Metric> {
        let inner = self.inner.read().unwrap();
        inner.metrics.clone()
    }
}
