use std::sync::atomic::{AtomicI64, Ordering};

/// Counters are integer values you can increment and decrement.
#[derive(Debug)]
pub struct Counter {
    value: AtomicI64,
}

impl Counter {
    pub(crate) fn new() -> Counter {
        Counter {
            value: AtomicI64::new(0),
        }
    }

    pub fn inc(&self, n: i64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    pub fn dec(&self, n: i64) {
        self.value.fetch_sub(n, Ordering::Relaxed);
    }

    pub fn value(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }
}
