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

use opentelemetry::proto::collector::metrics::v1::{
    metrics_service_client::MetricsServiceClient, ExportMetricsServiceRequest,
    ExportMetricsServiceResponse,
};

#[derive(Builder, Debug)]
pub struct OtlpReporter {
    registry: Arc<MetricsRegistry>,
    url: String,
}

impl OtlpReporter {
    async fn start(&self) {
        let mut client = MetricsServiceClient::connect(self.url.clone());

        let request = tonic::Request::new(ExportMetricsServiceRequest::new());
        let response = client.export(request).await?;
    }
}
