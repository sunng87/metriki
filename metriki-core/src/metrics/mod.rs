mod meter;

pub enum Metric {
    Meter(meter::Meter),
    Timer,
    Gauge,
    Histogram,
}
