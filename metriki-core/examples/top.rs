use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use metriki_core::metrics::{Counter, Metric};
use metriki_core::{MetricsRegistry, MetricsSet};

#[derive(Debug)]
struct Top {
    counters: Vec<Arc<Counter>>,
    n: usize,
}

impl MetricsSet for Top {
    fn get_all(&self) -> HashMap<String, Metric> {
        let mut working_copy = self.counters.clone();
        working_copy.sort_by_key(|c| -c.value());

        working_copy
            .iter()
            .take(self.n)
            .cloned()
            .enumerate()
            .map(|(idx, c)| (format!("counter.top.{}", idx), c.into()))
            .collect::<HashMap<String, Metric>>()
    }
}

impl Top {
    fn new(total: usize, top: usize) -> Top {
        Top {
            counters: (0..total).map(|_| Metric::counter()).collect(),
            n: top,
        }
    }

    fn inc_n(&self, idx: usize, n: i64) {
        if let Some(c) = self.counters.get(idx) {
            c.inc(n);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mr = MetricsRegistry::new();
    let size = 10;
    let top = Arc::new(Top::new(size, 3));
    mr.register_metrics_set("top", top.clone());

    // random inc
    for i in 0..size {
        top.inc_n(i, (rand::random::<f64>() * 100f64) as i64);
    }

    let snapshots = mr.snapshots();

    println!("{:?}", snapshots);
    Ok(())
}
