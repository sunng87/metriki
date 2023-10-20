use std::sync::Arc;
use std::thread;
use std::time::Duration;

use derive_builder::Builder;
use log::{log, Level};
use metriki_core::metrics::*;
use metriki_core::MetricsRegistry;

#[derive(Builder, Debug)]
pub struct LogReporter {
    #[builder(default, setter(into))]
    prefix: String,
    registry: Arc<MetricsRegistry>,
    #[builder(default = "30")]
    interval_secs: u64,
    #[builder(default = "Level::Info")]
    level: Level,
}

impl LogReporter {
    pub fn start(self) {
        let looper = move || loop {
            let metrics = self.registry.snapshots();
            for (ref key, metric) in metrics {
                match metric {
                    Metric::Counter(c) => self.report_counter(key.key(), c.as_ref()),
                    Metric::Gauge(g) => self.report_gauge(key.key(), g.as_ref()),
                    Metric::Timer(t) => self.report_timer(key.key(), t.as_ref()),
                    Metric::Meter(m) => self.report_meter(key.key(), m.as_ref()),
                    Metric::Histogram(h) => self.report_histogram(key.key(), &h.snapshot()),
                }
            }

            thread::sleep(Duration::from_secs(self.interval_secs));
        };

        thread::spawn(looper);
    }

    fn report_meter(&self, name: &str, meter: &Meter) {
        log!(self.level, "{}{}.m1={}", self.prefix, name, meter.m1_rate());
        log!(self.level, "{}{}.m5={}", self.prefix, name, meter.m5_rate());
        log!(
            self.level,
            "{}{}.m15={}",
            self.prefix,
            name,
            meter.m15_rate()
        );
        log!(
            self.level,
            "{}{}.count={}",
            self.prefix,
            name,
            meter.count()
        );
    }

    fn report_gauge(&self, name: &str, gauge: &Gauge) {
        let value = gauge.value();
        log!(self.level, "{}{}.value={}", self.prefix, name, value);
    }

    fn report_histogram(&self, name: &str, snapshot: &HistogramSnapshot) {
        log!(
            self.level,
            "{}{}.p50={}",
            self.prefix,
            name,
            snapshot.quantile(0.5)
        );
        log!(
            self.level,
            "{}{}.p75={}",
            self.prefix,
            name,
            snapshot.quantile(0.75)
        );
        log!(
            self.level,
            "{}{}.p90={}",
            self.prefix,
            name,
            snapshot.quantile(0.9)
        );
        log!(
            self.level,
            "{}{}.p99={}",
            self.prefix,
            name,
            snapshot.quantile(0.99)
        );
        log!(
            self.level,
            "{}{}.p999={}",
            self.prefix,
            name,
            snapshot.quantile(0.999)
        );
        log!(self.level, "{}{}.max={}", self.prefix, name, snapshot.max());
        log!(self.level, "{}{}.min={}", self.prefix, name, snapshot.min());
        log!(
            self.level,
            "{}{}.mean={}",
            self.prefix,
            name,
            snapshot.mean()
        );
    }

    fn report_counter(&self, name: &str, c: &Counter) {
        log!(self.level, "{}{}.value={}", self.prefix, name, c.value());
    }

    fn report_timer(&self, name: &str, t: &Timer) {
        self.report_meter(name, t.rate());
        self.report_histogram(name, &t.latency());
    }
}
