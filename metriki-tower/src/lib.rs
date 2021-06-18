use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::{Future, FutureExt, TryFutureExt};
use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;
use tower_service::Service;

pub struct MetrikiMiddleware<S> {
    registry: Arc<MetricsRegistry>,
    inner: S,
}

impl<S, R> Service<R> for MetrikiMiddleware<S>
where
    S: Service<R>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        let registry = self.registry.clone();
        let timer = registry.timer("requests");
        let timer_ctx = TimerContextArc::start(timer);

        let f = self
            .inner
            .call(req)
            .map(|resp| {
                timer_ctx.stop();
                resp
            })
            .map_err(move |e| {
                registry.meter("requests.error").mark();
                e
            });

        Box::pin(f)
    }
}

// TODO: layer
