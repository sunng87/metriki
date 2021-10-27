use std::sync::Arc;

use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

#[derive(Builder, Debug)]
pub struct InfluxDbReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    url: String,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(setter(into))]
    database: String,
    #[builder(default, setter(into))]
    username: Option<String>,
    #[builder(default, setter(into))]
    password: Option<String>,
    #[builder(default, setter(into))]
    measurement_prefix: String,
    #[builder(default, setter)]
    tags: HashMap<String, String>,
    #[builder(default = "50")]
    batch_size: usize,
}
