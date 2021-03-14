use std::sync::Arc;

use derive_builder::Builder;
use metriki_core::MetricsRegistry;
use prometheus::proto::{Metric, MetricFamily};
use prometheus::{Encoder, TextEncoder};

#[derive(Builder)]
pub struct PromethuesExporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    host: String,
    #[builder(setter)]
    port: u16,
}

impl PromethuesExporter {
    pub fn start(self) {}
}
