use std::collections::HashMap;

use metriki_core::metrics::Metric;
use metriki_core::MetricsSet;
use tikv_jemalloc_ctl::{epoch, stats};

#[derive(Debug)]
pub struct JemallocMetricsSet {
    prefix: &'static str,
}

impl MetricsSet for JemallocMetricsSet {
    fn get_all(&self) -> HashMap<String, Metric> {
        let mut result = HashMap::new();

        epoch::advance().unwrap();

        let active_gauge = Metric::gauge(Box::new(|| stats::active::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.active", self.prefix), active_gauge);

        let allocated_gauge =
            Metric::gauge(Box::new(|| stats::allocated::read().unwrap() as f64)).into();
        result.insert(
            format!("{}.jemalloc.allocated", self.prefix),
            allocated_gauge,
        );

        let metadata_gauge =
            Metric::gauge(Box::new(|| stats::metadata::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.metadata", self.prefix), metadata_gauge);

        let mapped_gauge = Metric::gauge(Box::new(|| stats::mapped::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.mapped", self.prefix), mapped_gauge);

        let resident_gauge =
            Metric::gauge(Box::new(|| stats::resident::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.resident", self.prefix), resident_gauge);

        let retained_gauge =
            Metric::gauge(Box::new(|| stats::retained::read().unwrap() as f64)).into();
        result.insert(format!("{}.jemalloc.retained", self.prefix), retained_gauge);

        result
    }
}
