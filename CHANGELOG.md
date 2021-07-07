# Change Log

## Tower [0.1.0] - 2021-07-07

Initial release of tower instrument. The module provides `MetricsLayer` and 
`MetricsService` for tower applications. Some built-in metrics are provided by default:

* A timer to measure latency and rate
* A meter to measure error rate

## Core [1.6.0] - 2021-07-06

### Added

* Added Timer API `TimerContextArc` to work with an `Arc` reference of a timer [#36]

## Core [1.5.0] - 2021-06-17

### Added

* New timer API `scoped` to measure execution of a closure with this timer
* `TimerContext` is exposed

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
