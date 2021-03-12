use std::fmt;

pub type GaugeFn = Box<dyn Fn() -> f64 + Send + Sync>;

/// Gauges are used to measure the instantaneous value of something.
pub struct Gauge {
    func: GaugeFn,
}

impl Gauge {
    pub(crate) fn new(f: GaugeFn) -> Gauge {
        Gauge { func: f }
    }

    pub fn value(&self) -> f64 {
        (*self.func)()
    }
}

impl fmt::Debug for Gauge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Gauge").finish()
    }
}
