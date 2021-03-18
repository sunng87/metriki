use std::sync::Arc;
use std::thread;

use derive_builder::Builder;
use log::warn;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;
use prometheus::proto::{
    Counter as PromethuesCount, Gauge as PromethuesGauge, Metric as PrometheusMetric, MetricFamily,
    MetricType, Quantile, Summary,
};
use prometheus::{Encoder, TextEncoder};
use tiny_http::{Response, Server};

#[derive(Builder)]
pub struct PrometheusExporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into), default = "\"0.0.0.0\".to_string()")]
    host: String,
    #[builder(setter)]
    port: u16,
    #[builder(default, setter(into))]
    prefix: String,
}

fn new_counter(v: f64) -> PrometheusMetric {
    let mut counter = PromethuesCount::new();
    counter.set_value(v);

    let mut metric = PrometheusMetric::new();
    metric.set_counter(counter);

    metric
}

fn new_gauge(v: f64) -> PrometheusMetric {
    let mut gauge = PromethuesGauge::new();
    gauge.set_value(v);

    let mut metric = PrometheusMetric::new();
    metric.set_gauge(gauge);

    metric
}

fn new_quantile(f: f64, s: &HistogramSnapshot) -> Quantile {
    let mut q = Quantile::new();

    q.set_quantile(f);
    q.set_value(s.quantile(f) as f64);

    q
}

impl PrometheusExporter {
    pub fn start(self) {
        let addr = format!("{}:{}", self.host, self.port);
        let server = Server::http(addr).expect("Failed to start promethues exporter server.");
        let encoder = TextEncoder::new();

        let looper = move || loop {
            if let Ok(req) = server.recv() {
                let metrics = self.registry.snapshots();
                let metric_families: Vec<MetricFamily> = metrics
                    .iter()
                    .map(|(ref key, metric)| match metric {
                        Metric::Counter(c) => self.report_counter(key, c.as_ref()),
                        Metric::Gauge(g) => self.report_gauge(key, g.as_ref()),
                        Metric::Timer(t) => self.report_timer(key, t.as_ref()),
                        Metric::Meter(m) => self.report_meter(key, m.as_ref()),
                        Metric::Histogram(h) => self.report_histogram(key, &h.snapshot()),
                    })
                    .collect();

                let mut buffer = Vec::new();
                encoder.encode(&metric_families, &mut buffer).unwrap();

                if let Err(e) = req.respond(Response::from_data(buffer)) {
                    warn!("Error on response {}", e);
                }
            }
        };

        thread::spawn(looper);
    }

    fn new_metric_family(&self, name: &str, mtype: MetricType) -> MetricFamily {
        let mut family = MetricFamily::new();
        family.set_name(format!("{}{}", self.prefix, name));
        family.set_field_type(mtype);

        family
    }

    fn report_meter(&self, name: &str, meter: &Meter) -> MetricFamily {
        let mut family = self.new_metric_family(name, MetricType::COUNTER);

        let counter = new_counter(meter.count() as f64);

        family.set_metric(vec![counter].into());
        family
    }

    fn report_gauge(&self, name: &str, gauge: &Gauge) -> MetricFamily {
        let mut family = self.new_metric_family(name, MetricType::GAUGE);

        let metric = new_gauge(gauge.value());
        family.set_metric(vec![metric].into());
        family
    }

    fn report_histogram(&self, name: &str, snapshot: &HistogramSnapshot) -> MetricFamily {
        let mut family = self.new_metric_family(name, MetricType::SUMMARY);

        let mut metric = PrometheusMetric::new();
        let quantiles = vec![
            new_quantile(0.5, snapshot),
            new_quantile(0.75, snapshot),
            new_quantile(0.9, snapshot),
            new_quantile(0.99, snapshot),
            new_quantile(0.999, snapshot),
        ];
        let mut summary = Summary::new();
        summary.set_quantile(quantiles.into());
        metric.set_summary(summary);
        family.set_metric(vec![metric].into());
        family
    }

    fn report_counter(&self, name: &str, c: &Counter) -> MetricFamily {
        let mut family = self.new_metric_family(name, MetricType::COUNTER);

        let counter = new_counter(c.value() as f64);

        family.set_metric(vec![counter].into());
        family
    }

    fn report_timer(&self, name: &str, t: &Timer) -> MetricFamily {
        let rate = t.rate();
        let latency = t.latency();

        let mut family = self.new_metric_family(name, MetricType::SUMMARY);
        let mut metric = PrometheusMetric::new();
        let quantiles = vec![
            new_quantile(0.5, &latency),
            new_quantile(0.75, &latency),
            new_quantile(0.9, &latency),
            new_quantile(0.99, &latency),
            new_quantile(0.999, &latency),
        ];
        let mut summary = Summary::new();
        summary.set_quantile(quantiles.into());
        summary.set_sample_count(rate.count());
        metric.set_summary(summary);
        family.set_metric(vec![metric].into());
        family
    }
}
