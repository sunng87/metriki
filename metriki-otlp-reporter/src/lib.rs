use std::sync::Arc;

use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

use derive_builder::Builder;

pub mod opentelemetry {
    tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
}

#[derive(Builder, Debug)]
pub struct InfluxDbReporter {
    registry: Arc<MetricsRegistry>,
}
