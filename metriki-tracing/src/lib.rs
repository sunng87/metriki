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
        // do nothing
    }

    fn record_follows_from(&self, _: &tracing::Id, _: &tracing::Id) {
        // do nothing
    }

    fn event(&self, event: &tracing::Event<'_>) {
        // register event as meter
        self.registry.meter(event.metadata().name()).mark();
    }

    fn enter(&self, span_id: &tracing::Id) {
        let span = self.current_span();
        if let Some(metadata) = span.metadata() {
            let name = metadata.name();
            let timer = self.registry.timer(name);
            let timer_ctx = TimerContextArc::start(timer);

            let mut timer_purgator = self.active_timers.lock().unwrap();
            timer_purgator.insert(span_id.clone(), timer_ctx);
        }
    }

    fn exit(&self, span_id: &tracing::Id) {
        let mut timer_purgator = self.active_timers.lock().unwrap();
        if let Some(timer_ctx) = timer_purgator.remove(span_id) {
            timer_ctx.stop();
        }
    }
}
