use std::fmt::Debug;

use super::Metric;

pub trait MetricsSet: Debug {
    fn get_all(&self) -> Vec<Metric>;
}
