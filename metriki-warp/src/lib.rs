//! # Metriki integration for Warp
//!
//! This module provides access for `MetricsRegistry` in filter. It also ships a
//! default wrapper that tracks some runtime metrics of your warp applicatoin.
//!
//! The built-in metrics includes:
//!
//! *
//!

use std::sync::Arc;

use http::method::Method;
use metriki_core::MetricsRegistry;
use warp::{self, Filter};

/// Inject metriki `MetricRegistry` into your warp filter
///
/// ```no_run
/// use std::sync::Arc;
///
/// use metriki_core::MetricsRegistry;
/// use metriki_warp::with_metrics;
/// use warp::{self, Filter};
///
/// let metrics = Arc::new(MetricsRegistry::new());
/// let router = warp::get()
///   .and(with_metrics(metrics))
///   .map(|mtk: Arc<MetricsRegistry>| {
///      mtk.meter("hit").mark();
///      "yes"
///   });
/// ```
///
pub fn with_metrics(
    m: Arc<MetricsRegistry>,
) -> impl Filter<Extract = (Arc<MetricsRegistry>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || m.clone())
}

pub fn metrics_wrapped<F, T>(
    filter: F,
    m: Arc<MetricsRegistry>,
) -> impl Filter<Extract = (T,)> + Clone + Send + Sync + 'static
where
    F: Filter<Extract = (T,), Error = std::convert::Infallible> + Clone + Send + Sync + 'static,
    F::Extract: warp::Reply,
{
    warp::method()
        .map(move |method: Method| {
            let ctx_all = m.timer("warp.requests").start();
            let ctx_method = m.timer(&format!("warp.{}", method.as_str())).start();
            vec![ctx_all, ctx_method]
        })
        .and(filter)
        .map(|resp| {
            for t in timers {
                t.stop();
            }

            match resp {
                Ok(_) => m.meter("warp.response.2xx").mark(),
                Err(e) if e.status().is_server_error() => m.meter("warp.response.5xx").mark(),
                Err(e) if e.status().is_client_error() => m.meter("warp.response.4xx").mark(),
                _ => {}
            }

            resp
        })
}
