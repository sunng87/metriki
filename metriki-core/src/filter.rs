use crate::metrics::Metric;

pub trait MetricsFilter: Send + Sync {
    fn accept(&self, name: &str, metric: &Metric) -> bool;
}
