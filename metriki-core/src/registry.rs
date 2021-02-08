use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::metrics::Metric;

pub struct MetricsRegistry {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    metrics: HashMap<String, Metric>,
}

impl MetricsRegistry {}
