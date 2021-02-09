use std::sync::Arc;

mod meter;

#[derive(Clone, Debug)]
pub enum Metric {
    Meter(Arc<Meter>),
    Timer,
    Gauge,
    Histogram,
}

pub use meter::Meter;
