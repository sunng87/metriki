use std::sync::Arc;
use std::thread;
use std::time::Duration;

use derive_builder::Builder;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

#[derive(Builder, Debug)]
pub struct InfluxDBReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    url: String,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(setter(into))]
    database: String,
    #[builder(setter(into))]
    username: Option<String>,
    #[builder(setter(into))]
    password: Option<String>,
}
