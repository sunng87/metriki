use std::sync::Arc;
use std::time::Duration;

use metriki_core::global::global_registry;
use metriki_jemalloc::JemallocMetricsSet;
use metriki_log_reporter::LogReporterBuilder;
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    env_logger::init();
    global_registry().register_metrics_set(
        "memory",
        Arc::new(JemallocMetricsSet::new("example.memory")),
    );

    LogReporterBuilder::default()
        .registry(global_registry())
        .interval_secs(5)
        .build()
        .unwrap()
        .start();

    let mut vecs: Vec<Vec<u8>> = Vec::new();
    loop {
        vecs.push(Vec::with_capacity(4 * 1024));
        std::thread::sleep(Duration::from_secs(1));
    }
}
