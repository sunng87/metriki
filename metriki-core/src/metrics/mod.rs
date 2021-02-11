use std::sync::Arc;

mod histogram;
mod meter;

#[derive(Clone, Debug)]
pub enum Metric {
    Meter(Arc<Meter>),
    Timer,
    Gauge,
    Histogram(Arc<Histogram>),
    Counter,
}

pub use histogram::Histogram;
pub use meter::Meter;
