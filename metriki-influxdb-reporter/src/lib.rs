use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use derive_builder::Builder;
use influxdb::{Client, InfluxDbWriteable, Timestamp, WriteQuery};

use log::warn;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;
use tokio::time::{sleep, Duration};

#[derive(Builder, Debug)]
pub struct InfluxDbReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into))]
    url: String,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(setter(into))]
    database: String,
    #[builder(default, setter(into))]
    username: Option<String>,
    #[builder(default, setter(into))]
    password: Option<String>,
    #[builder(default, setter(into))]
    measurement_prefix: String,
    #[builder(default, setter)]
    tags: HashMap<String, String>,
    #[builder(default = "50")]
    batch_size: usize,
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
        let looper = move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("can not create tokio runtime");
            runtime.block_on(async {
                loop {
                    let metrics = self.registry.snapshots();
                    let client = self.new_client();
                    let queries: Vec<WriteQuery> = metrics
                        .iter()
                        .map(|(key, metric)| match metric {
                            Metric::Counter(c) => self.report_counter(key, c.as_ref()),
                            Metric::Gauge(g) => self.report_gauge(key, g.as_ref()),
                            Metric::Timer(t) => self.report_timer(key, t.as_ref()),
                            Metric::Meter(m) => self.report_meter(key, m.as_ref()),
                            Metric::Histogram(h) => self.report_histogram(key, &h.snapshot()),
                        })
                        .collect();

                    if !queries.is_empty() {
                        self.do_query(&client, queries).await;
                    }

                    sleep(Duration::from_secs(self.interval_secs)).await;
                }
            })
        };
        std::thread::spawn(looper);
    }

    #[inline]
    fn measurement(&self, name: &str) -> String {
        format!("{}{}", self.measurement_prefix, name)
    }

    #[inline]
    fn with_query(&self, name: &str) -> WriteQuery {
        let mut query =
            Timestamp::Milliseconds(system_time_millis()).into_query(self.measurement(name));

        for (k, v) in self.tags.iter() {
            query = query.add_tag(k, v.clone());
        }

        query
    }

    #[inline]
    async fn do_query(&self, client: &Client, query: Vec<WriteQuery>) {
        // send query by chunk to avoid influxdb max request entity
        // error
        let chunks = query.chunks(self.batch_size);
        for ch in chunks {
            let batch = ch.to_owned();
            if let Err(e) = client.query(batch).await {
                warn!("Failed to write influxdb, {}", e)
            }
        }
    }

    fn report_meter(&self, name: &str, meter: &Meter) -> WriteQuery {
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
