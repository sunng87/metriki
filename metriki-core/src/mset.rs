use std::collections::HashMap;
use std::fmt::Debug;

use crate::metrics::Metric;

pub trait MetricsSet: Debug {
    fn get_all(&self) -> HashMap<String, Metric>;
}
