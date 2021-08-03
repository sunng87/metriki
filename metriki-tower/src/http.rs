use std::sync::Arc;
use std::task::{Context, Poll};

use derive_builder::Builder;
use futures::{FutureExt, TryFutureExt};
use hyper::{Body, Request, Response};
use metriki_core::metrics::TimerContextArc;
use metriki_core::MetricsRegistry;
use tower_layer::Layer;
use tower_service::Service;

use crate::common::ResultFuture;

#[derive(Debug, Clone)]
pub struct HyperMetricsService<S> {
    registry: Arc<MetricsRegistry>,
    base_metric_name: String,
    inner: S,
}

// Request {
//     method: GET,
//     uri: /,
//     version: HTTP/1.1,
//     headers: {
//         "host": "localhost:3000",
//         "user-agent": "curl/7.78.0",
//         "accept": "*/*",
//     },
//     body: Body(
//         Empty,
//     ),
// }

impl<S> Service<Request<Body>> for HyperMetricsService<S>
where
    S: Service<Request<Body>> + Send,
    S::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = ResultFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let registry = self.registry.clone();
        let name = self.base_metric_name.clone();

        let request_timer = registry.timer(&format!("{}.all", name));
        let method_timer = registry.timer(&format!("{}.{}", name, req.method().as_str()));
        let request_timer_ctx = TimerContextArc::start(request_timer);
        let method_timer_ctx = TimerContextArc::start(method_timer);

        registry.counter(&format!("{}.inflight", name)).inc(1);

        let f = self
            .inner
            .call(req)
            .map(|resp| {
                // timers
                request_timer_ctx.stop();
                method_timer_ctx.stop();

                // inflight request counter
                registry.counter(&format!("{}.inflight", name)).dec(1);

                if let Ok(resp) = resp {
                    // meters by status code family, 2xx, 3xx, 4xx and 5xx
                    let status_family = resp.status_code().as_u16() / 100;
                    registry
                        .meter(&format!("{}.{}xx", name, status_family))
                        .mark();
                }

                resp
            })
            .map_err(move |e| {
                // error meter
                registry.meter(&format!("{}.error", name)).mark();

                // inflight request counter
                registry.counter(&format!("{}.inflight", name)).dec(1);

                e
            });

        Box::pin(f)
    }
}

#[derive(Builder, Debug, Clone)]
pub struct HyperMetricsLayer {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into), default = "\"requests\".to_owned()")]
    base_metric_name: String,
}

impl<S> Layer<S> for HyperMetricsLayer {
    type Service = HyperMetricsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        HyperMetricsService {
            registry: self.registry.clone(),
            inner: service,
            base_metric_name: self.base_metric_name.clone(),
        }
    }
}
