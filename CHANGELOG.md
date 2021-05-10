# Change Log

## Core [1.2.0] - 2021-05-10

### Added

* `MetricsSet` APIs

### Changed

* Histogram is now backed by HdrHistogram algorithm

## Core [1.1.0] - 2021-04-27

### Added

* `count` API for `Histogram`
* `meanRate` API for `Meter`
* Feature `ser` for serialization support of metric types
* New `MetricsFilter` API to filter metrics from reporters and exporters
