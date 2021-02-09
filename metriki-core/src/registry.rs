use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::metrics::{Meter, Metric};

#[derive(Default, Debug)]
pub struct MetricsRegistry {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default, Debug)]
struct Inner {
    metrics: HashMap<String, Metric>,
}

impl MetricsRegistry {
    /// Return `Meter` that has been registered or just created and resgitered.
    /// Panic if a metric is already register but is not meter
    pub fn meter(&self, name: &str) -> Arc<Meter> {
        let mut inner = self.inner.lock().unwrap();

        if !inner.metrics.contains_key(name) {
            let meter = Meter::new();
            inner
                .metrics
                .insert(name.to_owned(), Metric::Meter(Arc::new(meter)));
        }

        let metric = inner.metrics.get(name).unwrap();
        match metric {
            Metric::Meter(ref m) => m.clone(),
            _ => panic!("A metric with same name and different type is already registered."),
        }
    }
}
