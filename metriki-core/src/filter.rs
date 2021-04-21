use crate::metrics::Metric;

/// A filter to include/exclude some metrics based on its name,
/// type or actual data.
///
pub trait MetricsFilter: Send + Sync {
    fn accept(&self, name: &str, metric: &Metric) -> bool;
}
