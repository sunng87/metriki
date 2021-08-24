//! # Metriki Jemalloc Instrumentation
//!
//! This library provide metrics of jemalloc memory allocator.
//! The data is fetched from a library called `jemalloc-ctl`, and we are using the [tikv
//! maintained version of it](https://github.com/tikv/jemallocator). It is recommended
//! to use that version of jemallocator too.
//!
//! The instrumentation is provided as a `MetricsSet`. Use `MetricsRegistry::register_metrics_set` to add it to your metriki registry.
//!
//! [An example](https://github.com/sunng87/metriki/blob/master/metriki-jemalloc/examples/jemalloc.rs) can be found in our github repo.

use std::collections::HashMap;

use metriki_core::metrics::Metric;
use metriki_core::MetricsSet;
use tikv_jemalloc_ctl::{epoch, stats};

/// The MetricsSet that provides gauges of jemalloc data.
///
/// Currently, the data is fetched from `jemalloc_ctl::stats`, including:
///
/// - `prefix.jemalloc.active`: bytes of active pages
/// - `prefix.jemalloc.allocated`: total allocated bytes
/// - `prefix.jemalloc.metadata`: jemalloc metadata bytes
/// - `prefix.jemalloc.mapped`: bytes in active extents mapped by the allocator
/// - `prefix.jemalloc.resident`: bytes in active extents mapped by the allocator
/// - `prefix.jemalloc.retianed`: bytes in physically resident data pages mapped by the allocator.
#[derive(Debug)]
pub struct JemallocMetricsSet {
    prefix: &'static str,
}

impl JemallocMetricsSet {
    /// Create a `JemallocMetricsSet` and specify a prefix for its metrics names.
    pub fn new(prefix: &'static str) -> JemallocMetricsSet {
        JemallocMetricsSet { prefix }
    }
}

impl MetricsSet for JemallocMetricsSet {
    fn get_all(&self) -> HashMap<String, Metric> {
        let mut result = HashMap::new();

        epoch::advance().unwrap();

        let active = Metric::gauge(Box::new(|| stats::active::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.active", self.prefix), active);

        let allocated = Metric::gauge(Box::new(|| stats::allocated::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.allocated", self.prefix), allocated);

        let metadata = Metric::gauge(Box::new(|| stats::metadata::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.metadata", self.prefix), metadata);

        let mapped = Metric::gauge(Box::new(|| stats::mapped::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.mapped", self.prefix), mapped);

        let resident = Metric::gauge(Box::new(|| stats::resident::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.resident", self.prefix), resident);

        let retained = Metric::gauge(Box::new(|| stats::retained::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.retained", self.prefix), retained);

        result
    }
}
