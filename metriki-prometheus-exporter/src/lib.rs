use std::sync::Arc;

use derive_builder::Builder;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;
use prometheus::proto::{
    Counter as PromethuesCount, Gauge as PromethuesGauge, Metric, MetricFamily, MetricType,
    Quantile, Summary,
};
use prometheus::{Encoder, TextEncoder};

#[derive(Builder)]
pub struct PromethuesExporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    host: String,
    #[builder(setter)]
    port: u16,
    #[builder(default, setter(into))]
    prefix: String,
}

fn new_counter(v: f64) -> Metric {
    let mut counter = PromethuesCount::new();
    counter.set_value(v);

    let mut metric = Metric::new();
    metric.set_counter(counter);

    metric
}

fn new_gauge(v: f64) -> Metric {
    let mut gauge = PromethuesGauge::new();
    gauge.set_value(v);

    let mut metric = Metric::new();
    metric.set_gauge(gauge);

    metric
}

fn new_quantile(f: f64, s: &HistogramSnapshot) -> Quantile {
    let mut q = Quantile::new();

    q.set_quantile(f);
    q.set_value(s.quantile(f) as f64);

    q
}

impl PromethuesExporter {
    pub fn start(self) {}

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

        let mut metric = Metric::new();
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
        let mut metric = Metric::new();
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
