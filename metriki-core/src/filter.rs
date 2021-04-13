pub trait MetricsFilter {
    fn accept(&self, name: &str) -> bool;
}
