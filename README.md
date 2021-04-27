# Metriki

Metriki is a rust library ported from [Dropwizard Metrics](https://github.com/dropwizard/metrics).

Like Dropwizard Metrics, Metriki aggregates metrics on client-side and outputs limit amount data.
For now, it uses exponential decay and moving average algorithms to keep the memory footprint at
a low level, while retaining the statistical information.

## Features and TODOs

- Metrics [(doc)](https://docs.rs/metriki-core/) [(crate)](https://crates.io/crates/metriki-core)
- Reporters / Exporters
  - [x] logger [(doc)](https://docs.rs/metriki-log-reporter/) [(crate)](https://crates.io/crates/metriki-log-reporter)
  - [x] influxdb [(doc)](https://docs.rs/metriki-influxdb-reporter/) [(crate)](https://crates.io/crates/metriki-influxdb-reporter)
  - [x] riemann [(doc)](https://docs.rs/metriki-riemann-reporter/) [(crate)](https://crates.io/crates/metriki-riemann-reporter)
  - [x] prometheus [(doc)](https://docs.rs/metriki-prometheus-exporter/) [(crate)](https://crates.io/crates/metriki-promethes-exporter)
- Instruments
  - [x] warp [(doc)](https://docs.rs/metriki-warp/) [(crate)](https://crates.io/crates/metriki-warp)
  - [ ] ?reqwest

## Concepts

### Metrics

- **Counter**: a value that can be increased and decreased.
- **Meter**: measures rate of an event.
- **Histogram**: records distribution of data over time.
- **Timer**: a combination of meter and histogram.
- **Gauge**: a function that provides value when queried.

### Reporter

A component to report metric data periodically. Typically used for data sinks which has a push-model.

### Exporter

A component to expose metric data to external queriers. Typically for pull based data sinks.

## License

MIT/Apache-2.0
