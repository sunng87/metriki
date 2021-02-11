use std::sync::Arc;

mod counter;
mod histogram;
mod meter;
mod timer;

#[derive(Clone, Debug)]
pub enum Metric {
    Meter(Arc<Meter>),
    Timer(Arc<Timer>),
    Gauge,
    Histogram(Arc<Histogram>),
    Counter(Arc<Counter>),
}

pub use counter::Counter;
pub use histogram::{Histogram, HistogramSnapshot};
pub use meter::Meter;
pub use timer::Timer;
