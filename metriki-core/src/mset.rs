use std::collections::HashMap;
use std::fmt::Debug;

use crate::metrics::Metric;

pub trait MetricsSet: Send + Sync + Debug {
    fn get_all(&self) -> HashMap<String, Metric>;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::MetricsSet;
    use crate::metrics::Metric;
    use crate::registry::MetricsRegistry;

    #[derive(Debug)]
    struct DummyMetricsSet;

    impl MetricsSet for DummyMetricsSet {
        fn get_all(&self) -> HashMap<String, Metric> {
            let counter = Metric::counter();
            counter.inc(10);

            let mut map: HashMap<String, Metric> = HashMap::new();
            map.insert("test.set.counter".to_owned(), counter.into());

            map
        }
    }

    #[test]
    fn test_metrics_set() {
        let registry = MetricsRegistry::new();
        registry.register_metrics_set("dummy", Box::new(DummyMetricsSet));
        registry.counter("test.default.counter").inc(1);

        let snapshots = registry.snapshots();

        assert_eq!(2, snapshots.len());
        assert!(snapshots.get("test.set.counter").is_some());
        assert!(snapshots.get("test.default.counter").is_some());
    }
}
