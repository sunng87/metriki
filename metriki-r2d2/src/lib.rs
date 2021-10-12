//! # Metriki Instrumentation for r2d2
//!
//! This library provides extensions for r2d2, which is generic
//! database connection pool library, to measure performance for
//! database applications.
//!
//! It provides following metriki metrics:
//!
//! * `r2d2.checkout`: A meter records the rate of your application
//! borrowing connection from the pool
//! * `r2d2.wait`: A histogram summarizes the distribution of time
//! spent on borrowing connection from the pool
//! * `r2d2.timeout`: A meter records the error rate of timeout
//! borrowing connection
//! * `r2d2.usage`: A histogram summarizes the distribution of time
//! for using the connection. Typically this is the time spent to
//! query your database.
//!
//! ## Usage
//!
//! Add MetrikiHandler as `r2d2::Builder`
//!
//! ```rust,ignore
//! // Create metriki event handler from metriki global registry
//! let metriki_handler = MetrikiHandlerBuilder::default()
//!     .registry(global_registry())
//!     .build()
//!     .unwrap();
//!
//! let manager = r2d2_foodb::FooConnectionManager::new("localhost:1234");
//! let pool = r2d2::Pool::builder()
//!     .max_size(15)
//!     // set event handler to the builder
//!     .event_handler(Box::new(metriki_handler))
//!     .build(manager)
//!     .unwrap();
//! ```
//!
//! ## diesel Support
//!
//! The Rust ORM library diesel has an re-exported version of r2d2. By
//! enabling `diesel` feature of `metriki-r2d2`, it will work with the
//! diesel variant.
//!
//! ## Customization
//!
//! The metric name prefix `r2d2` can be customized with
//! `MetrikiHandlerBuilder` by setting `name`. This is required when
//! you have multiple r2d2 pools in your application.
//!
use std::sync::Arc;

use derive_builder::Builder;
#[cfg(not(feature = "diesel"))]
use r2d2::{
    event::{CheckinEvent, CheckoutEvent, TimeoutEvent},
    HandleEvent,
};

#[cfg(feature = "diesel")]
use diesel::r2d2::{
    event::{CheckinEvent, CheckoutEvent, TimeoutEvent},
    HandleEvent,
};

use metriki_core::MetricsRegistry;

// The r2d2 EventHandler that tracks usage of the database connection
// and its connection pool.
#[derive(Debug, Builder)]
pub struct MetrikiHandler {
    registry: Arc<MetricsRegistry>,
    #[builder(setter(into), default = "\"r2d2\".to_owned()")]
    name: String,
}

impl HandleEvent for MetrikiHandler {
    fn handle_checkout(&self, event: CheckoutEvent) {
        self.registry
            .meter(&format!("{}.checkout", self.name))
            .mark();
        self.registry
            .histogram(&format!("{}.wait", self.name))
            .update(event.duration().as_millis() as u64);
    }

    fn handle_timeout(&self, _event: TimeoutEvent) {
        self.registry
            .meter(&format!("{}.timeout", self.name))
            .mark();
    }

    fn handle_checkin(&self, event: CheckinEvent) {
        self.registry
            .histogram(&format!("{}.usage", self.name))
            .update(event.duration().as_millis() as u64);
    }
}
