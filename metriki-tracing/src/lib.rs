use std::sync::Arc;

use tracing::Subscriber;
use tracing::{Event, Id};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::LookupSpan,
};

use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;

// A tracing layer that tracks span and events with Metriki timers
// and meters.
pub struct MetrikiLayer {
    registry: Arc<MetricsRegistry>,
}

impl<S> Layer<S> for MetrikiLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // register event as meter
        self.registry.meter(event.metadata().name()).mark();
    }

    fn on_enter(&self, _id: &Id, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.lookup_current() {
            let metadata = span_ref.metadata();
            let name = metadata.name();
            let timer = self.registry.timer(name);
            let timer_ctx = TimerContextArc::start(timer);

            span_ref.extensions_mut().insert(timer_ctx);
        }
    }

    fn on_exit(&self, _id: &Id, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.lookup_current() {
            if let Some(timer_ctx) = span_ref.extensions().get::<TimerContextArc>() {
                timer_ctx.stop();
            }
        }
    }
}

impl MetrikiLayer {
    // Create `MetrikiLayer` from a Metriki MetricsRegistry
    pub fn new(registry: Arc<MetricsRegistry>) -> MetrikiLayer {
        MetrikiLayer {
            registry: registry.clone(),
        }
    }
}
