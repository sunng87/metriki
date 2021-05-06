use std::collections::HashMap;
use std::fmt::Debug;

use crate::metrics::Metric;

pub trait MetricsSet: Send + Sync + Debug {
    fn get_all(&self) -> HashMap<String, Metric>;
}
