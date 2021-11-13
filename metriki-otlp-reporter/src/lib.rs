use std::sync::Arc;
use std::thread;
use std::time::Duration;

use derive_builder::Builder;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;
use tokio::runtime::Builder;

mod opentelemetry;

use opentelemetry::proto::collector::metrics::v1::{
    metrics_service_client::MetricsServiceClient, ExportMetricsServiceRequest,
    ExportMetricsServiceResponse,
};
use opentelemetry::proto::common::v1::InstrumentationLibrary;
use opentelemetry::proto::metrics::v1::{InstrumentationLibraryMetrics, ResourceMetrics};

static METRIKI_INSTRUMENTATION_LIBRARY: InstrumentationLibrary = InstrumentationLibrary {
    name: "metriki".to_string(),
    version: "1".to_string(),
};

static SCHEMA_URL: String = "http://metriki.schema".to_owned();

#[derive(Builder, Debug)]
pub struct OtlpReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    url: String,
    #[builder(default = "30")]
    interval_secs: u64,
}

impl OtlpReporter {
    pub fn start(&self) {
        let looper = move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            let url = self.url.clone();
            loop {
                let mut client = rt.block_on(MetricsServiceClient::connect(url)).unwrap();
                let request = tonic::Request::new(self.create_export_request());
                // TODO: export
                let response = rt.block_on(client.export(request)).unwrap();

                thread::sleep(Duration::from_secs(self.interval_secs));
            }
        };

        thread::spawn(looper);
    }

    fn create_export_request(&self) -> ExportMetricsServiceRequest {
        let metrics = self.registry.snapshots();

        let resource_metrics: Vec<ResourceMetrics> = metrics
            .iter()
            .map(|(key, metric)| match metric {
                Metric::Counter(c) => self.report_counter(key, c.as_ref()),
                Metric::Gauge(g) => self.report_gauge(key, g.as_ref()),
                Metric::Timer(t) => self.report_timer(key, t.as_ref()),
                Metric::Meter(m) => self.report_meter(key, m.as_ref()),
                Metric::Histogram(h) => self.report_histogram(key, &h.snapshot()),
            })
            .collect();

        ExportMetricsServiceRequest { resource_metrics }
    }

    fn create_resource_metrics() -> ResourceMetrics {
        // instrumentation data
        // metrics

        ResourceMetrics {
            resource: None,
            instrumentation_library_metrics: Vec::new(),
            // TODO:
            schema_url: SCHEMA_URL,
        }
    }

    fn create_instrumentation_metrics() -> InstrumentationLibraryMetrics {
        InstrumentationLibraryMetrics {
            instrumentation_library: Some(METRIKI_INSTRUMENTATION_LIBRARY),
            metrics: Vec::new(),
            schema_url: SCHEMA_URL,
        }
    }

    fn report_meter(&self, name: &str, meter: &Meter) -> ResourceMetrics {
        self.with_query(name)
            .add_field("m1", meter.m1_rate())
            .add_field("m5", meter.m5_rate())
            .add_field("m15", meter.m15_rate())
    }

    fn report_gauge(&self, name: &str, gauge: &Gauge) -> WriteQuery {
        let value = gauge.value();
        self.with_query(name).add_field("value", value)
    }

    fn report_histogram(&self, name: &str, snapshot: &HistogramSnapshot) -> WriteQuery {
        self.with_query(name)
            .add_field("p50", snapshot.quantile(0.5))
            .add_field("p75", snapshot.quantile(0.75))
            .add_field("p90", snapshot.quantile(0.90))
            .add_field("p99", snapshot.quantile(0.99))
            .add_field("p999", snapshot.quantile(0.999))
            .add_field("min", snapshot.min())
            .add_field("max", snapshot.max())
            .add_field("mean", snapshot.mean())
    }

    fn report_counter(&self, name: &str, c: &Counter) -> WriteQuery {
        self.with_query(name).add_field("value", c.value())
    }

    fn report_timer(&self, name: &str, t: &Timer) -> WriteQuery {
        let rate = t.rate();
        let latency = t.latency();

        self.with_query(name)
            .add_field("p50", latency.quantile(0.5))
            .add_field("p75", latency.quantile(0.75))
            .add_field("p90", latency.quantile(0.90))
            .add_field("p99", latency.quantile(0.99))
            .add_field("p999", latency.quantile(0.999))
            .add_field("min", latency.min())
            .add_field("max", latency.max())
            .add_field("mean", latency.mean())
            .add_field("m1", rate.m1_rate())
            .add_field("m5", rate.m5_rate())
            .add_field("m15", rate.m15_rate())
    }
}
