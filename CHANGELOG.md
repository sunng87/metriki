# Change Log

## Core [1.4.0] - 2021-06-02

### Added

* Macro features: built-in macros `#[timed]` and `#[metered]`.

## Core [1.3.0] - 2021-05-13

### Added

* Global instance of `MetricRegistry` is added at `metriki_core::global::global_registry()`.

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
