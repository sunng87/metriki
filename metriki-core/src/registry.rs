use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::sync::{Arc, RwLock};

use crate::filter::MetricsFilter;
use crate::metrics::*;

/// Entrypoint of all metrics
///
#[derive(Default)]
pub struct MetricsRegistry {
    inner: Arc<RwLock<Inner>>,
    filter: Option<Box<dyn MetricsFilter + 'static>>,
}

impl Debug for MetricsRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("MetricsRegistry")
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Default, Debug)]
struct Inner {
    metrics: HashMap<String, Metric>,
}

impl MetricsRegistry {
    /// Create a default metrics registry
    pub fn new() -> MetricsRegistry {
        MetricsRegistry::default()
    }

    /// Create a default metrics registry wrapped in an Arc.
    pub fn arc() -> Arc<MetricsRegistry> {
        Arc::new(MetricsRegistry::default())
    }

    /// Return `Meter` that has been registered and create if not found.
    ///
    /// Meter a metric to measure rate of an event. It will report rate in 1 minute,
    /// 5 minutes and 15 minutes, which is similar to Linux load.
    ///
    /// This function may panic if a metric is already registered with type other than meter.
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

    /// Return `Histogram` that has been registered and create if not found.
    ///
    /// Histogram a metric to measure distribution of a series of data. The distribution will
    /// be reported with `max`, `min`, `mean`, `stddev` and the value at particular percentile.
    ///
    /// This function may panic if a metric is already registered with type other than histogram.
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

    /// Return `Counter` that has been registered and create if not found.
    ///
    /// Counter a metric to measure the number of some state.
    ///
    /// This function may panic if a metric is already registered with type other than counter.
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

    /// Return `Timer` that has been registered and create if not found.
    ///
    /// Timer is a combination of meter and histogram. The meter part is to track rate of
    /// the event. And the histogram part maintains the distribution of time spent for the event.
    ///
    /// This function may panic if a metric is already registered with type other than counter.
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

    /// Register a `Gauge` with given function.
    ///
    /// The guage will return a value when any reporter wants to fetch data from it.
    pub fn gauge(&self, name: &str, func: GaugeFn) {
        let mut inner = self.inner.write().unwrap();
        inner
            .metrics
            .insert(name.to_owned(), Metric::Gauge(Arc::new(Gauge::new(func))));
    }

    /// Returns all the metrics hold in the registry.
    ///
    /// This is useful for reporters to fetch all values from the registry.
    pub fn snapshots(&self) -> HashMap<String, Metric> {
        let inner = self.inner.read().unwrap();
        if let Some(ref filter) = self.filter {
            let mut results = HashMap::new();
            for (k, v) in inner.metrics.iter() {
                if filter.accept(k, v) {
                    results.insert(k.to_owned(), v.clone());
                }
            }
            results
        } else {
            inner.metrics.clone()
        }
    }

    pub fn set_filter(&mut self, filter: Box<dyn MetricsFilter + 'static>) {
        self.filter = Some(filter);
    }
}

#[cfg(test)]
mod test {
    use crate::filter::MetricsFilter;
    use crate::metrics::Metric;
    use crate::registry::MetricsRegistry;

    #[test]
    fn test_metrics_filter() {
        let mut registry = MetricsRegistry::new();

        registry.meter("l1.tomcat.request").mark();
        registry.meter("l1.jetty.request").mark();
        registry.meter("l2.tomcat.request").mark();
        registry.meter("l2.jetty.request").mark();

        struct NameFilter;
        impl MetricsFilter for NameFilter {
            fn accept(&self, name: &str, _: &Metric) -> bool {
                name.starts_with("l1")
            }
        }

        registry.set_filter(Box::new(NameFilter));

        let snapshot = registry.snapshots();
        assert_eq!(2, snapshot.len());
    }
}
