use std::sync::Arc;

use lazy_static::lazy_static;

use crate::registry::MetricsRegistry;

lazy_static! {
    static ref GLOBAL_REGISTRY: Arc<MetricsRegistry> = MetricsRegistry::arc();
}

/// Provide access to built-in global instance of `MetricsRegistry`.
pub fn global_registry() -> Arc<MetricsRegistry> {
    GLOBAL_REGISTRY.clone()
}
