use std::sync::{Arc, RwLock};

use hdrhistogram::Histogram as HdrHistogram;

#[cfg(feature = "ser")]
use serde::ser::SerializeMap;
#[cfg(feature = "ser")]
use serde::{Serialize, Serializer};

const DEFAULT_RANGE_MAX: u64 = 3600 * 24;

/// Histograms are used to record the distribution of data over time.
///
/// By default, `Histogram` uses HdrHistogram for better data accuracy
/// and smaller memory footprint.
#[derive(Debug)]
pub struct Histogram {
    inner: Arc<RwLock<HdrHistogram<u64>>>,
}

#[derive(Debug)]
pub struct HistogramSnapshot {
    inner: Arc<RwLock<HdrHistogram<u64>>>,
}

impl Histogram {
    pub(crate) fn new() -> Histogram {
        let inner = HdrHistogram::<u64>::new_with_bounds(1, DEFAULT_RANGE_MAX, 2).unwrap();

        Histogram {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn update(&self, value: u64) {
        let mut inner = self.inner.write().unwrap();
        let value = value.min(DEFAULT_RANGE_MAX);
        // ignore the error
        inner.record(value).ok();
    }

    pub fn snapshot(&self) -> HistogramSnapshot {
        let snapshot = self.inner.clone();
        HistogramSnapshot { inner: snapshot }
    }
}

impl HistogramSnapshot {
    pub fn count(&self) -> u64 {
        self.inner.read().unwrap().len()
    }

    pub fn mean(&self) -> f64 {
        self.inner.read().unwrap().mean()
    }

    pub fn max(&self) -> u64 {
        self.inner.read().unwrap().max()
    }

    pub fn min(&self) -> u64 {
        self.inner.read().unwrap().min()
    }

    pub fn stddev(&self) -> f64 {
        self.inner.read().unwrap().stdev()
    }

    pub fn quantile(&self, quantile: f64) -> u64 {
        self.inner.read().unwrap().value_at_quantile(quantile)
    }
}

#[cfg(feature = "ser")]
impl Serialize for Histogram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(9))?;
        let snapshot = self.snapshot();

        map.serialize_entry("mean", &snapshot.mean())?;
        map.serialize_entry("max", &snapshot.max())?;
        map.serialize_entry("min", &snapshot.min())?;
        map.serialize_entry("stddev", &snapshot.stddev())?;

        map.serialize_entry("p50", &snapshot.quantile(0.5))?;
        map.serialize_entry("p75", &snapshot.quantile(0.75))?;
        map.serialize_entry("p90", &snapshot.quantile(0.9))?;
        map.serialize_entry("p99", &snapshot.quantile(0.99))?;
        map.serialize_entry("p999", &snapshot.quantile(0.999))?;

        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::{Histogram, DEFAULT_RANGE_MAX};

    #[test]
    fn test_histogram_range() {
        let histogram = Histogram::new();

        histogram.update(0);
        histogram.update(1);
        histogram.update(1000);
        histogram.update(DEFAULT_RANGE_MAX + 1);

        let snapshot = histogram.snapshot();

        assert_eq!(4, snapshot.count());
        assert_eq!(0, snapshot.min());
        // assert_eq!(DEFAULT_RANGE_MAX, snapshot.max());
    }
}
