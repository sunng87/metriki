use std::sync::Arc;

use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

use derive_builder::Builder;

pub mod opentelemetry {
    pub mod proto {
        pub mod collector {
            pub mod metrics {
                pub mod v1 {
                    tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
                }
            }
        }
        pub mod common {
            pub mod v1 {
                tonic::include_proto!("opentelemetry.proto.common.v1");
            }
        }

        pub mod metrics {
            pub mod v1 {
                tonic::include_proto!("opentelemetry.proto.metrics.v1");
            }
        }

        pub mod resource {
            pub mod v1 {
                tonic::include_proto!("opentelemetry.proto.resource.v1");
            }
        }
    }
}

#[derive(Builder, Debug)]
pub struct InfluxDbReporter {
    registry: Arc<MetricsRegistry>,
}
