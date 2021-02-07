use std::collections::HashMap;

use crate::metrics::Metric;

pub struct MetricsRegistry {
    metrics: HashMap<String, Metric>,
}

impl MetricsRegistry {}
