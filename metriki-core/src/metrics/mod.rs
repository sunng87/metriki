use std::sync::Arc;

mod counter;
mod histogram;
mod meter;

#[derive(Clone, Debug)]
pub enum Metric {
    Meter(Arc<Meter>),
    Timer,
    Gauge,
    Histogram(Arc<Histogram>),
    Counter(Arc<Counter>),
}

pub use counter::Counter;
pub use histogram::Histogram;
pub use meter::Meter;
