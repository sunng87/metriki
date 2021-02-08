pub struct Meter {}

impl Meter {
    pub fn mark(&self) {}

    pub fn mark_n(&self, n: u64) {}

    pub fn m1_rate(&self) -> f64 {
        0f64
    }

    pub fn m5_rate(&self) -> f64 {
        0f64
    }

    pub fn m15_rate(&self) -> f64 {
        0f64
    }
}
