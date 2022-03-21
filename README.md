# Metriki

Metriki is a rust library ported from [Dropwizard
Metrics](https://github.com/dropwizard/metrics).

Like Dropwizard Metrics, Metriki aggregates metrics on client-side and
outputs limited amount data.

## Quick Start

```rust
// create a timer to track the execution rate and latency of this function
// to use macros you will need to turn on `macros` feature of metriki_core
#[timed]
fn your_function() {
  // a function metered by a timer for its rate and latency
}

use metriki_core::global::global_registry;
use metriki_influxdb_reporter::InfluxDbReporterBuilder;

// by default, the timer is registered in this global_registry()
let registry = global_registry();

// start a reporter to send data into influxdb
InfluxDbReporterBuilder::default()
  .registry(registry.clone())
  .build()
  .start();

```

## Components

- Metrics [(doc)](https://docs.rs/metriki-core/) [(crate)](https://crates.io/crates/metriki-core)
- Reporters / Exporters
  - [x] logger [(doc)](https://docs.rs/metriki-log-reporter/) [(crate)](https://crates.io/crates/metriki-log-reporter)
  - [x] influxdb [(doc)](https://docs.rs/metriki-influxdb-reporter/) [(crate)](https://crates.io/crates/metriki-influxdb-reporter)
  - [x] riemann [(doc)](https://docs.rs/metriki-riemann-reporter/) [(crate)](https://crates.io/crates/metriki-riemann-reporter)
  - [x] prometheus [(doc)](https://docs.rs/metriki-prometheus-exporter/) [(crate)](https://crates.io/crates/metriki-promethes-exporter)
  - [x] statsd [(doc)](https://docs.rs/metriki-statsd-reporter/) [(crate)](https://crates.io/crates/metriki-statsd-reporter)
- Instruments
  - [x] jemalloc: tracking jemalloc stats
        [(doc)](https://docs.rs/metriki-jemalloc/)
        [(crate)](https://crates.io/crates/metriki-jemalloc).
  - [x] tracing: tracing subscriber layer
        [(doc)](https://docs.rs/metriki-tracing/)
        [(crate)](https://crates.io/crates/metriki-tracing).
  - [x] tower + hyper: tower layer for metriki integration
        [(doc)](https://docs.rs/metriki-tower/)
        [(crate)](https://crates.io/crates/metriki-tower).
  - [x] warp: warp middleware to inject metriki `MetricsRegistry`
        [(doc)](https://docs.rs/metriki-warp/)
        [(crate)](https://crates.io/crates/metriki-warp).
  - [x] r2d2: monitoring database connection usage
        [(doc)](https://docs.rs/metriki-r2d2/)
        [(crate)](https://crates.io/crates/metriki-r2d2).
  - [x] tokio: monitoring tokio internals using
        [tokio_metrics](https://github.com/tokio/tokio-metrics)
        [(doc)](https://docs.rs/metriki-tokio/)
        [(crate)](https://crates.io/crates/metriki-tokio).

## Concepts

### Metrics

- **Counter**: a value that can be increased and decreased.
- **Meter**: measures rate of an event.
- **Histogram**: records distribution of data over time.
- **Timer**: a combination of meter and histogram.
- **Gauge**: a function that provides value when queried.

### MetricsRegistry

An entrypoint and holder of all metrics.

### MetricsSet

A trait to be implemented so that dynamic metrics can be added into
registry. Metrics from the set are pulled into registry everytime when
reporters and exporters pulling values from the registry.

### Reporter

A component to report metric data periodically. Typically used for
data sinks which has a push-model.

### Exporter

A component to expose metric data to external queriers. Typically for
pull based data sinks.

## License

MIT/Apache-2.0
