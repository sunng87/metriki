#[macro_use]
extern crate metriki_macros;

use metriki_core::global::global_registry;

#[timed(name = "my.example.f1")]
fn my_example_f1() {
    dbg!("456");
}

#[test]
fn test_my_example_f1() {
    my_example_f1();

    let s = global_registry().snapshots();
    assert_eq!(1, s.len());
    assert!(s.contains_key("my.example.f1"));
}
