#![cfg_attr(docsrs, feature(doc_cfg))]
//! # metriki-core
//!
//! Metriki-core is a metrics library that ported from Coda Hale's Dropwizard Metrics.
//!
//! This library heavily relies on exponential moving average and HDR histogram for its
//! meter and histogram implementation. So it won't stop all the samples in memory and
//! works great on application with heavy load.
//!
//! Currently the library supports five kinds of metrics, includes:
//!
//! * Meter: a measure for rate, useful for tracking QPS, error rate, etc.
//! * Histogram: distribution of a series of numerical data
//! * Timer: a combination of meter and histogram, for tracking latency and rate at the same time
//! * Counter: a value can be increased and decreased
//! * Gauge: a function that reports a value when it is called
//! * MetricsSet: a trait to be implemented and to give dynamic metrics when called by registry
//!
//! ## Ecosystem
//!
//! ### Reporters and Exporters
//!
//! Like Dropwizard Metrics, reporters are components that fetch data from registry and push
//! to some destination.
//!
//! A [Log reporter](https://github.com/sunng87/metriki/tree/master/metriki-log-reporter) is
//! the reference implementation.
//!
//! `Exporters` are components that serve metrics data for pull-based services, like Promethus.
//!
//! ### Instruments
//!
//! We will try to integrate metriki with some common libraries/frameworks of Rust ecosystem,
//! includes web frameworks, net programming frameworks, database connectors, etc.
//!
//! ## Usage
//!
//! Create a `MetricsRegistry` for your application as the entrypoint and holder of all metrics.
//!
//! Metriki allows you to create multiple registry that serves different metrics and reporters.
//! However, for most cases, you can use the built-in global registry
//! `metriki_core::global::global_registry` as a singleton instance for all your application.
//!
//! ```
//! use metriki_core::MetricsRegistry;
//!
//! let registry = MetricsRegistry::new();
//!
//! // using meter: mark an event as it happened once
//! registry.meter("event").mark();
//!
//! // record a sample value 42 into my_data series
//! registry.histogram("my_data").update(42);
//!
//! // increase my_counter by 1
//! registry.counter("my_counter").inc(1);
//!
//! // start a timer and record its rate
//! let my_timer = registry.timer("my_timer");
//! let timer_context = my_timer.start();
//! // stop the timer and record its data
//! timer_context.stop();
//!
//! // register a gauge function
//! registry.gauge("my_gauge", Box::new(|| {
//!   42.0
//! }))
//! ```
//!
//! ## Macros
//!
//! Metriki ships attribute macros `timed` and `metered` to track function execution.
//!
//! This can be turned on with `macros` feature enabled.
//!
//! ```skip
//! // A timer `my.example.fn1` is created to track rate and latency of `example_fn1` calls
//! #[timed(name="my.example.fn1")]
//! fn example_fn1() {
//!   // ...
//! }
//!
//! // A meter `my.example.fn2` is created to track rate of `example_fn2` calls
//! #[metered(name="my.example.fn2")]
//! fn example_fn2() {
//!   // ...
//! }
//!
//! ```
//!
//!

mod filter;
pub mod global;
pub mod metrics;
mod mset;
mod registry;
mod utils;

pub use filter::MetricsFilter;
pub use mset::MetricsSet;
pub use registry::MetricsRegistry;

#[cfg(feature = "macros")]
pub use metriki_macros::{metered, timed};
