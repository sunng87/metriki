# Metriki

Metriki is a rust library ported from [Dropwizard Metrics](https://github.com/dropwizard/metrics).

## Features and TODOs

- Metrics [(doc)](https://docs.rs/metriki-core/) [(crate)](https://crates.io/crates/metriki-core)
  - [x] meter
  - [x] histogram
  - [x] timer
  - [x] gauge
  - [x] counter
- Reporters / Exporters
  - [x] logger [(doc)](https://docs.rs/metriki-log-reporter/) [(crate)](https://crates.io/crates/metriki-log-reporter)
  - [x] influxdb [(doc)](https://docs.rs/metriki-influxdb-reporter/) [(crate)](https://crates.io/crates/metriki-influxdb-reporter)
  - [x] riemann [(doc)](https://docs.rs/metriki-riemann-reporter/) [(crate)](https://crates.io/crates/metriki-riemann-reporter)
  - [ ] prometheus
- Instruments
  - [x] warp [(doc)](https://docs.rs/metriki-warp/) [(crate)](https://crates.io/crates/metriki-warp)
  - [ ] reqwest

## License

MIT/Apache-2.0
