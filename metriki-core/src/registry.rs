use dashmap::DashMap;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

use crate::filter::MetricsFilter;
use crate::key::{Key, Tag};
use crate::metrics::*;
use crate::mset::MetricsSet;

/// Entrypoint of all metrics
///
#[derive(Default)]
pub struct MetricsRegistry {
    inner: Arc<Inner>,
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
    metrics: DashMap<Key, Metric>,
    mset: DashMap<String, Arc<dyn MetricsSet + 'static>>,
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
    /// # Panics
    ///
    /// This function may panic if a metric is already registered with type other than meter.
    pub fn meter(&self, name: &str) -> Arc<Meter> {
        let key = Key::from_name(name);
        self.do_meter(key)
    }

    pub fn meter_with_tags(&self, name: &str, tags: Vec<Tag>) -> Arc<Meter> {
        let key = Key::from(name, tags);
        self.do_meter(key)
    }

    fn do_meter(&self, key: Key) -> Arc<Meter> {
        let meter = {
            self.inner
                .metrics
                .get(&key)
                .as_deref()
                .map(|metric| match metric {
                    Metric::Meter(ref m) => m.clone(),
                    _ => {
                        panic!("A metric with same name and different type is already registered.")
                    }
                })
        };

        if let Some(m) = meter {
            m
        } else {
            let meter = Arc::new(Meter::new());
            self.inner.metrics.insert(key, Metric::Meter(meter.clone()));
            meter
        }
    }


    /// Return `Histogram` that has been registered and create if not found.
    ///
    /// Histogram a metric to measure distribution of a series of data. The distribution will
    /// be reported with `max`, `min`, `mean`, `stddev` and the value at particular percentile.
    ///
    /// # Panics
    ///
    /// This function may panic if a metric is already registered with type other than histogram.
    pub fn histogram(&self, name: &str) -> Arc<Histogram> {
        let key = Key::from_name(name);
        self.do_histogram(key)
    }

    pub fn histogram_with_tags(&self, name: &str, tags: Vec<Tag>) -> Arc<Histogram> {
        let key = Key::from(name, tags);
        self.do_histogram(key)
    }

    fn do_histogram(&self, key: Key) -> Arc<Histogram> {
        let histo = {
            self.inner
                .metrics
                .get(&key)
                .as_deref()
                .map(|metric| match metric {
                    Metric::Histogram(ref m) => m.clone(),
                    _ => {
                        panic!("A metric with same name and different type is already registered.")
                    }
                })
        };

        if let Some(m) = histo {
            m
        } else {
            let histo = Arc::new(Histogram::new());
            self.inner
                .metrics
                .insert(key, Metric::Histogram(histo.clone()));
            histo
        }
    }

    /// Return `Counter` that has been registered and create if not found.
    ///
    /// Counter a metric to measure the number of some state.
    ///
    /// # Panics
    ///
    /// This function may panic if a metric is already registered with type other than counter.
    pub fn counter(&self, name: &str) -> Arc<Counter> {
        let key = Key::from_name(name);
        self.do_counter(key)
    }

    pub fn counter_with_tags(&self, name: &str, tags: Vec<Tag>) -> Arc<Counter> {
        let key = Key::from(name, tags);
        self.do_counter(key)
    }

    fn do_counter(&self, key: Key) -> Arc<Counter> {
        let counter = {
            self.inner
                .metrics
                .get(&key)
                .as_deref()
                .map(|metric| match metric {
                    Metric::Counter(ref m) => m.clone(),
                    _ => {
                        panic!("A metric with same name and different type is already registered.")
                    }
                })
        };

        if let Some(m) = counter {
            m
        } else {
            let counter = Arc::new(Counter::new());
            self.inner
                .metrics
                .insert(key, Metric::Counter(counter.clone()));
            counter
        }
    }

    /// Return `Timer` that has been registered and create if not found.
    ///
    /// Timer is a combination of meter and histogram. The meter part is to track rate of
    /// the event. And the histogram part maintains the distribution of time spent for the event.
    ///
    /// # Panics
    ///
    /// This function may panic if a metric is already registered with type other than counter.
    pub fn timer(&self, name: &str) -> Arc<Timer> {
        let key = Key::from_name(name);
        self.do_timer(key)
    }

    pub fn timer_with_tags(&self, name: &str, tags: Vec<Tag>) -> Arc<Timer> {
        let key = Key::from(name, tags);
        self.do_timer(key)
    }

    fn do_timer(&self, key: Key) -> Arc<Timer> {
        let timer = {
            self.inner
                .metrics
                .get(&key)
                .as_deref()
                .map(|metric| match metric {
                    Metric::Timer(ref m) => m.clone(),
                    _ => {
                        panic!("A metric with same name and different type is already registered.")
                    }
                })
        };

        if let Some(m) = timer {
            m
        } else {
            let timer = Arc::new(Timer::new());
            self.inner.metrics.insert(key, Metric::Timer(timer.clone()));
            timer
        }
    }

    /// Register a `Gauge` with given function.
    ///
    /// The guage will return a value when any reporter wants to fetch data from it.
    pub fn gauge(&self, name: &str, func: Box<dyn GaugeFn>) {
        let key = Key::from_name(name);
        self.do_gauge(key, func)
    }

    pub fn gauge_with_tags(&self, name: &str, tags: Vec<Tag>, func: Box<dyn GaugeFn>) {
        let key = Key::from(name, tags);
        self.do_gauge(key, func)
    }

    fn do_gauge(&self, key: Key, func: Box<dyn GaugeFn>) {
        self.inner
            .metrics
            .insert(key, Metric::Gauge(Arc::new(Gauge::new(func))));
    }

    /// Returns all the metrics hold in the registry.
    /// Metrics is filtered if a filter is set for this registry.
    ///
    /// This is useful for reporters to fetch all values from the registry.
    pub fn snapshots(&self) -> HashMap<Key, Metric> {
        let filter = self.filter.as_ref();
        let mut results: HashMap<Key, Metric> = HashMap::new();

        let metrics = self.inner.metrics.clone();
        for (k, v) in metrics.into_read_only().iter() {
            if filter.map(|f| f.accept(k.name.as_str(), v)).unwrap_or(true) {
                results.insert(k.to_owned(), v.clone());
            }
        }
        let mset = self.inner.mset.clone();
        for metrics_set in mset.into_read_only().values() {
            let metrics = metrics_set.get_all();
            for (k, v) in metrics.iter() {
                if filter.map(|f| f.accept(k, v)).unwrap_or(true) {
                    results.insert(Key::from_name(k), v.clone());
                }
            }
        }

        results
    }

    /// Set a filter for this registry.
    /// The filter will apply to `snapshots` function.
    ///
    pub fn set_filter(&mut self, filter: Option<Box<dyn MetricsFilter + 'static>>) {
        self.filter = filter;
    }

    /// Register a MetricsSet implementation.
    ///
    /// A MetricsSet returns a set of metrics when `snapshots()` is called on
    /// the registry. This provides dynamic metrics that can be added into registry
    /// based custom rules.
    ///
    /// The name has nothing to do with metrics it added to `snapshots()` results.
    /// It's just for identify the metrics set for dedup and removal.
    pub fn register_metrics_set(&self, name: &str, mset: Arc<dyn MetricsSet + 'static>) {
        self.inner.mset.insert(name.to_owned(), mset);
    }

    /// Unregister a MetricsSet implementation by its name.
    pub fn unregister_metrics_set(&self, name: &str) {
        self.inner.mset.remove(name);
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

        registry.set_filter(Some(Box::new(NameFilter)));

        let snapshot = registry.snapshots();
        assert_eq!(2, snapshot.len());
    }
}

#[cfg(feature = "ser")]
impl Serialize for MetricsRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let snapshot = self.snapshots();
        let mut map = serializer.serialize_map(Some(snapshot.len()))?;

        for (k, v) in snapshot.iter() {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}
