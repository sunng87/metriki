use std::sync::Arc;

use metriki_core::MetricsRegistry;
use warp::{self, Filter};

/// Inject metriki `MetricRegistry` into your warp handler
///
/// ```no_run
/// let metrics = Arc::new(MetricsRegistry::new());
/// warp::get()
///   .and(with_metrics(metrics))
///   .and_then(|mtk: Arc<MetricsRegistry| move {
///      mtk.meter("hit").mark();
///      Ok("yes")
///   })
/// ```
///
pub fn with_metrics(
    m: Arc<MetricsRegistry>,
) -> impl Filter<Extract = (Arc<MetricsRegistry>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || m.clone())
}
