use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use derive_builder::Builder;

use lazy_static::lazy_static;
use log::warn;
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;
use rustmann::protos::riemann::Event;
use rustmann::{EventBuilder, RiemannClient, RiemannClientOptionsBuilder};
use tokio::time;

lazy_static! {
    static ref THE_HOSTNAME: Option<String> = hostname::get()
        .ok()
        .and_then(|o| o.to_str().map(|s| s.to_owned()));
}

#[derive(Builder, Debug)]
pub struct RiemannReporter {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into), default = "\"localhost\".to_string()")]
    host: String,
    #[builder(setter, default = "5555")]
    port: u16,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(default, setter)]
    tags: Vec<String>,
}

fn system_time_millis() -> u128 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
    timestamp
        .expect("System time earlier than UNIX_EPOCH")
        .as_millis()
}

impl RiemannReporter {
    fn new_client(&self) -> RiemannClient {
        let riemann_options = RiemannClientOptionsBuilder::default()
            .host(&self.host)
            .port(self.port)
            .build();

        RiemannClient::new(&riemann_options)
    }

    pub fn start(self) {
        tokio::spawn(async move {
            loop {
                let metrics = self.registry.snapshots();
                let client = Arc::new(self.new_client());

                let events: Vec<Event> = metrics
                    .iter()
                    .flat_map(|(key, metric)| match metric {
                        Metric::Counter(c) => self.report_counter(key, c.as_ref()).into_iter(),
                        Metric::Gauge(g) => self.report_gauge(key, g.as_ref()).into_iter(),
                        Metric::Timer(t) => self.report_timer(key, t.as_ref()).into_iter(),
                        Metric::Meter(m) => self.report_meter(key, m.as_ref()).into_iter(),
                        Metric::Histogram(h) => {
                            self.report_histogram(key, &h.snapshot()).into_iter()
                        }
                    })
                    .collect();

                if !events.is_empty() {
                    if let Err(e) = client.send_events(events).await {
                        warn!("Failed to write influxdb, {}", e);
                    }
                }

                time::sleep(Duration::from_secs(self.interval_secs)).await;
            }
        });
    }

    fn event(&self) -> EventBuilder {
        let mut eb = EventBuilder::new().time(system_time_millis() as i64);

        if let Some(the_host) = THE_HOSTNAME.as_ref() {
            eb = eb.host(the_host);
        }

        for t in &self.tags {
            eb = eb.add_tag(t);
        }

        eb
    }

    fn report_meter(&self, name: &str, meter: &Meter) -> Vec<Event> {
        vec![
            self.event()
                .service(format!("{}.m1", name))
                .metric_d(meter.m1_rate())
                .build(),
            self.event()
                .service(format!("{}.m5", name))
                .metric_d(meter.m5_rate())
                .build(),
            self.event()
                .service(format!("{}.m15", name))
                .metric_d(meter.m15_rate())
                .build(),
        ]
    }

    fn report_gauge(&self, name: &str, gauge: &Gauge) -> Vec<Event> {
        let value = gauge.value();
        vec![self.event().service(name).metric_d(value).build()]
    }

    fn report_histogram(&self, name: &str, snapshot: &HistogramSnapshot) -> Vec<Event> {
        vec![
            self.event()
                .service(format!("{}.p50", name))
                .metric_d(snapshot.quantile(0.5) as f64)
                .build(),
            self.event()
                .service(format!("{}.p75", name))
                .metric_d(snapshot.quantile(0.75) as f64)
                .build(),
            self.event()
                .service(format!("{}.p90", name))
                .metric_d(snapshot.quantile(0.9) as f64)
                .build(),
            self.event()
                .service(format!("{}.p99", name))
                .metric_d(snapshot.quantile(0.99) as f64)
                .build(),
            self.event()
                .service(format!("{}.p999", name))
                .metric_d(snapshot.quantile(0.999) as f64)
                .build(),
            self.event()
                .service(format!("{}.min", name))
                .metric_d(snapshot.min() as f64)
                .build(),
            self.event()
                .service(format!("{}.max", name))
                .metric_d(snapshot.max() as f64)
                .build(),
            self.event()
                .service(format!("{}.mean", name))
                .metric_d(snapshot.mean())
                .build(),
        ]
    }

    fn report_counter(&self, name: &str, c: &Counter) -> Vec<Event> {
        vec![self
            .event()
            .service(name)
            .metric_d(c.value() as f64)
            .build()]
    }

    fn report_timer(&self, name: &str, t: &Timer) -> Vec<Event> {
        let rate = t.rate();
        let latency = t.latency();

        vec![
            self.event()
                .service(format!("{}.p50", name))
                .metric_d(latency.quantile(0.5) as f64)
                .build(),
            self.event()
                .service(format!("{}.p75", name))
                .metric_d(latency.quantile(0.75) as f64)
                .build(),
            self.event()
                .service(format!("{}.p90", name))
                .metric_d(latency.quantile(0.9) as f64)
                .build(),
            self.event()
                .service(format!("{}.p99", name))
                .metric_d(latency.quantile(0.99) as f64)
                .build(),
            self.event()
                .service(format!("{}.p999", name))
                .metric_d(latency.quantile(0.999) as f64)
                .build(),
            self.event()
                .service(format!("{}.min", name))
                .metric_d(latency.min() as f64)
                .build(),
            self.event()
                .service(format!("{}.max", name))
                .metric_d(latency.max() as f64)
                .build(),
            self.event()
                .service(format!("{}.mean", name))
                .metric_d(latency.mean())
                .build(),
            self.event()
                .service(format!("{}.m1", name))
                .metric_d(rate.m1_rate())
                .build(),
            self.event()
                .service(format!("{}.m5", name))
                .metric_d(rate.m5_rate())
                .build(),
            self.event()
                .service(format!("{}.m15", name))
                .metric_d(rate.m15_rate())
                .build(),
        ]
    }
}
