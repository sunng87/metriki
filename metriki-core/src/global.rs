use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::registry::MetricsRegistry;

/// Provide access to built-in global instance of `MetricsRegistry`.
pub fn global_registry() -> Arc<MetricsRegistry> {
    static GLOBAL_REGISTRY: OnceCell<Arc<MetricsRegistry>> = OnceCell::new();

    GLOBAL_REGISTRY
        .get_or_init(|| MetricsRegistry::arc())
        .clone()
}

#[cfg(test)]
mod test {
    use super::global_registry;

    #[test]
    fn test_global_registry() {
        let registry = global_registry();
        registry.meter("hello").mark();

        assert!(registry.snapshots().len() > 0);
    }
}
