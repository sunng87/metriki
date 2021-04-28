use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cadence::prelude::*;
use cadence::{Metric as StatsdMetric, MetricBuilder, StatsdClient, UdpMetricSink};
use derive_builder::Builder;
use futures::executor;
use log::warn;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

#[derive(Builder, Debug)]
pub struct StatsdReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    host: String,
    #[builder(setter, default = cadence::DEFAULT_PORT)]
    port: u16,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(default, setter(into))]
    prefix: String,
    #[builder(default, setter)]
    tags: HashMap<String, String>,
}

fn system_time_millis() -> u128 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
    timestamp
        .expect("System time earlier than UNIX_EPOCH")
        .as_millis()
}

impl StatsdReporter {
    fn new_client(&self) -> StatsdClient {
        let host = (self.host, self.port);
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let sink = UdpMetricSink::from(host, socket).unwrap();
        StatsdClient::from_sink(&self.prefix, sink)
    }

    pub fn start(self) {
        let looper = move || loop {
            let metrics = self.registry.snapshots();
            let client = self.new_client();

            for (key, metric) in metrics {
                match metric {
                    Metric::Counter(ref c) => self.report_counter(&key, c, &client),
                    Metric::Gauge(ref g) => self.report_gauge(&key, g.as_ref(), &client),
                    Metric::Timer(ref t) => self.report_timer(&key, t.as_ref(), &client),
                    Metric::Meter(ref m) => self.report_meter(&key, m, &client),
                    Metric::Histogram(ref h) => self.report_histogram(&key, &h.snapshot(), &client),
                }
            }

            thread::sleep(Duration::from_secs(self.interval_secs));
        };

        thread::spawn(looper);
    }

    fn send<T>(&self, mb: MetricBuilder<'_, '_, T>)
    where
        T: StatsdMetric + From<String>,
    {
        for (k, v) in self.tags {
            mb = mb.with_tag(&k, &v);
        }

        mb.send();
    }

    fn report_meter(&self, name: &str, meter: &Meter, client: &StatsdClient) {
        self.send(client.meter_with_tags(format!("{}.m1_rate", name), meter.m1_rate() as u64));
        self.send(client.meter_with_tags(format!("{}.m5_rate", name), meter.m5_rate() as u64));
        self.send(client.meter_with_tags(format!("{}.m15_rate", name), meter.m15_rate() as u64));
        self.send(client.meter_with_tags(format!("{}.mean_rate", name), meter.mean_rate() as u64));
    }

    fn report_gauge(&self, name: &str, gauge: &Gauge, client: &StatsdClient) {
        let value = gauge.value();
        self.send(client.gauge_f64_with_tags(name, value));
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
