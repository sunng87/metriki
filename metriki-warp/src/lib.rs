use std::sync::Arc;

use metriki_core::MetricsRegistry;
use warp::{self, Filter};

/// Inject metriki `MetricRegistry` into your warp handler
///
/// ```no_run
/// use std::sync::Arc;
///
/// use metriki_core::MetricsRegistry;
/// use metriki_warp::with_metrics;
/// use warp::{self, Filter};
///
/// let metrics = MetricsRegistry::new();
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
