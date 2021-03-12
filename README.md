# Metriki

Metriki is a rust library ported from [Dropwizard Metrics](https://github.com/dropwizard/metrics).

This library is a work in progress.

## Features and TODOs

- Metrics [(doc)](https://docs.rs/metriki-core/)
  - [x] meter
  - [x] histogram
  - [x] timer
  - [x] gauge
  - [x] counter
- Reporters
  - [x] logger [(doc)](https://docs.rs/metriki-log-reporter/)
  - [x] influxdb [(doc)](https://docs.rs/metriki-influxdb-reporter/)
  - [x] riemann [(doc)](https://docs.rs/metriki-riemann-reporter/)
  - [ ] prometheus
- Instruments
  - [x] warp [(doc)](https://docs.rs/metriki-warp/)
  - [ ] reqwest

## Docs

- [metriki-core](https://docs.rs/metriki-core/)

## License

MIT/Apache-2.0
