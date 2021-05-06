//! # metriki-core
//!
//! Metriki-core is a metrics library that ported from Coda Hale's Dropwizard Metrics.
//!
//! This library heavily relies on exponential moving average and exponential decay algorithms
//! for its meter and histogram implementation. So it won't stop all the samples in memory and
//! works great on application with heavy load.
//!
//! Currently the library supports five kinds of metrics, includes:
//!
//! * Meter: a measure for rate, useful for tracking QPS, error rate, etc.
//! * Histogram: distribution of a series of numerical data
//! * Timer: a combination of meter and histogram, for tracking latency and rate at the same time
//! * Counter: just counter
//! * Gauge: a function that reports a value when it is called
//!
//! ## Ecosystem
//!
//! ### Reporters
//!
//! Like Dropwizard Metrics, reporters are component that fetches data from registry and sents
//! to some destination.
//!
//! A [Log reporter](https://github.com/sunng87/metriki/tree/master/metriki-log-reporter) is
//! the reference implementation.
//!
//! ### Integrations
//!
//! We will try to integrate metriki with some common libraries/frameworks of Rust ecosystem,
//! includes web frameworks, net programming frameworks, database connectors, etc.
//!
//! ## Usage
//!
//! Create a `MetricsRegistry` for your application as the entrypoint and holder of all metrics.
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

mod filter;
pub mod metrics;
mod mset;
mod registry;
mod utils;

pub use filter::MetricsFilter;
pub use mset::MetricsSet;
pub use registry::MetricsRegistry;
