use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use derive_builder::Builder;
use influxdb::{Client, InfluxDbWriteable, Query, Timestamp, WriteQuery};
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
    #[builder(setter(into))]
    username: Option<String>,
    #[builder(setter(into))]
    password: Option<String>,
    #[builder(default, setter(into))]
    measurement_prefix: String,
    //TODO: tags
}

fn system_time_millis() -> u128 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
    timestamp
        .expect("System time earlier than UNIX_EPOCH")
        .as_millis()
}

impl InfluxDbReporter {
    fn new_client(&self) -> Client {
        let client = Client::new(&self.url, &self.database);

        if let (Some(username), Some(password)) = (self.username.as_ref(), self.password.as_ref()) {
            client.with_auth(username, password)
        } else {
            client
        }
    }

    pub fn start(self) {
        let looper = move || loop {
            let metrics = self.registry.snapshots();
            let client = self.new_client();

            for (ref key, metric) in metrics {
                match metric {
                    Metric::Counter(c) => self.report_counter(&client, key, c.as_ref()),
                    Metric::Gauge(g) => self.report_gauge(&client, key, g.as_ref()),
                    Metric::Timer(t) => self.report_timer(&client, key, t.as_ref()),
                    Metric::Meter(m) => self.report_meter(&client, key, m.as_ref()),
                    Metric::Histogram(h) => self.report_histogram(&client, key, &h.snapshot()),
                }
            }

            thread::sleep(Duration::from_secs(self.interval_secs));
        };

        thread::spawn(looper);
    }

    #[inline]
    fn measurement(&self, name: &str) -> String {
        format!("{}{}", self.measurement_prefix, name)
    }

    fn report_meter(&self, client: &Client, name: &str, meter: &Meter) {
        let q = Timestamp::Milliseconds(system_time_millis())
            .into_query(self.measurement(name))
            .add_field("m1", meter.m1_rate())
            .add_field("m5", meter.m5_rate())
            .add_field("m15", meter.m15_rate());

        //TODO: query response
        client.query(&q);
    }

    fn report_gauge(&self, client: &Client, name: &str, gauge: &Gauge) {
        let value = gauge.value();
        let q = Timestamp::Milliseconds(system_time_millis())
            .into_query(self.measurement(name))
            .add_field("value", value);

        client.query(&q);
    }

    fn report_histogram(&self, client: &Client, name: &str, snapshot: &HistogramSnapshot) {
        let q = Timestamp::Milliseconds(system_time_millis())
            .into_query(self.measurement(name))
            .add_field("p50", snapshot.quantile(0.5))
            .add_field("p75", snapshot.quantile(0.75))
            .add_field("p90", snapshot.quantile(0.90))
            .add_field("p99", snapshot.quantile(0.99))
            .add_field("p999", snapshot.quantile(0.999))
            .add_field("min", snapshot.min())
            .add_field("max", snapshot.max())
            .add_field("mean", snapshot.mean());

        client.query(&q);
    }

    fn report_counter(&self, client: &Client, name: &str, c: &Counter) {
        let q = Timestamp::Milliseconds(system_time_millis())
            .into_query(self.measurement(name))
            .add_field("value", c.value());

        client.query(&q);
    }

    fn report_timer(&self, client: &Client, name: &str, t: &Timer) {
        let rate = t.rate();
        let latency = t.latency();

        let q = Timestamp::Milliseconds(system_time_millis())
            .into_query(self.measurement(name))
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
            .add_field("m15", rate.m15_rate());

        client.query(&q);
    }
}
