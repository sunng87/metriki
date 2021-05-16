#[macro_use]
extern crate metriki_macros;

use std::sync::Arc;

use metriki_core::global::global_registry;
use metriki_core::MetricsRegistry;

#[timed(name = "my.example.f1")]
fn my_example_f1() {
    dbg!("456");
}

#[timed]
fn my_example_f2() {
    dbg!("456");
}

#[timed(registry = "local_registry")]
fn my_example_f3(local_registry: Arc<MetricsRegistry>) {
    dbg!("456");
}

#[metered]
fn my_example_f4() {}

#[test]
fn test_my_example_f1() {
    my_example_f1();

    let s = global_registry().snapshots();
    assert!(s.contains_key("my.example.f1"));
}

#[test]
fn test_my_example_f2() {
    my_example_f2();

    let s = global_registry().snapshots();
    assert!(s.contains_key("my_example_f2"));
}

#[test]
fn test_my_example_f3() {
    let local_registry = MetricsRegistry::arc();
    my_example_f3(local_registry.clone());

    let s = local_registry.snapshots();
    assert!(s.contains_key("my_example_f3"));
}

#[test]
fn test_my_example_f4() {
    my_example_f4();

    let s = global_registry().snapshots();
    assert!(s.contains_key("my_example_f4"));
}
