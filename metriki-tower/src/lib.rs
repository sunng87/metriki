use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use derive_builder::Builder;
use futures::{Future, FutureExt, TryFutureExt};
use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;
use tower_layer::Layer;
use tower_service::Service;

pub struct MetricsService<S> {
    registry: Arc<MetricsRegistry>,
    base_metric_name: Option<String>,
    inner: S,
}

impl<S> MetricsService<S> {
    fn name(&self) -> String {
        self.base_metric_name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "requests".to_owned())
    }
}

type ResultFuture<R, E> = Pin<Box<dyn Future<Output = Result<R, E>>>>;

impl<S, R> Service<R> for MetricsService<S>
where
    S: Service<R>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResultFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        let registry = self.registry.clone();
        let name = self.name();
        let timer = registry.timer(&name);
        let timer_ctx = TimerContextArc::start(timer);

        let f = self
            .inner
            .call(req)
            .map(|resp| {
                timer_ctx.stop();
                resp
            })
            .map_err(move |e| {
                registry.meter(&format!("{}.error", name)).mark();
                e
            });

        Box::pin(f)
    }
}

/// The tower layer to generate tower services for Metriki
#[derive(Builder, Debug)]
pub struct MetricsLayer {
    registry: Arc<MetricsRegistry>,
    #[builder(default, setter(into))]
    base_metric_name: Option<String>,
}

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetricsService {
            registry: self.registry.clone(),
            inner: service,
            base_metric_name: self.base_metric_name.clone(),
        }
    }
}
