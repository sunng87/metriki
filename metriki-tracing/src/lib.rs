use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use tracing::Subscriber;

use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;

pub struct MetrikiSubscriber {
    registry: Arc<MetricsRegistry>,
    enabled: bool,
    active_timers: Arc<Mutex<HashMap<tracing::Id, TimerContextArc>>>,
    id_gen: AtomicU64,
}

impl Subscriber for MetrikiSubscriber {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        // check metadata to
        self.enabled
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::Id {
        let next_id = self.id_gen.fetch_add(1, Ordering::SeqCst);
        tracing::Id::from_u64(next_id)
    }

    fn record(&self, _: &tracing::Id, _: &tracing::span::Record<'_>) {
        todo!()
    }
    fn record_follows_from(&self, _: &tracing::Id, _: &tracing::Id) {
        todo!()
    }
    fn event(&self, _: &tracing::Event<'_>) {
        todo!()
    }
    fn enter(&self, _: &tracing::Id) {
        todo!()
    }
    fn exit(&self, _: &tracing::Id) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
