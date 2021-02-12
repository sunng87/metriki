use std::fmt;

pub type GaugeFn = Box<dyn Fn() -> f64 + Send + Sync>;

pub struct Gauge {
    func: GaugeFn,
}

impl Gauge {
    pub fn new(f: GaugeFn) -> Gauge {
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
