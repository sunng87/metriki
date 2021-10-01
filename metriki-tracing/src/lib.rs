//! # Metriki as a Tracing Backend
//!
//! This library acts as a backend for
//! [tracing](https://github.com/tokio-rs/tracing). It tracks spans
//! and events created with tracing API and translates them into
//! [metriki](https://github.com/sunng87/metriki) concepts.
//!
//! * Spans are recorded with metriki timers.
//! * Events are recorded with metriki meters.
//!
//! By using this backend, developers are able to add Metriki and its
//! exporter/reporter ecosystem into tracing without touching metriki
//! apis.
//!
use std::sync::Arc;

use tracing::Subscriber;
use tracing::{Event, Id};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::LookupSpan,
};

use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;

// A tracing layer to be integrated with tracing_subscriber's
// Registry.
//
// ```no_run
// // Create MetrikiLayer from metriki registry
// let layer = MetrikiLayer::new(global_registry());
//
// // Create a subscriber with tracing_subscriber's Registry and
// // configure MetrikiLayer with it
// let subscriber = Registry::default().with(layer);
//
// // Configure the subscriber to tracing
// tracing::subscriber::set_global_default(subscriber).unwrap();
// ```
//
pub struct MetrikiLayer {
    registry: Arc<MetricsRegistry>,
}

impl<S> Layer<S> for MetrikiLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // register event as meter
        // FIXME: better event metrics name
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
        MetrikiLayer { registry }
    }
}
