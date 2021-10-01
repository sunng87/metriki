use std::time::Duration;

use metriki_core::global::global_registry;
use metriki_log_reporter::LogReporterBuilder;
use metriki_tracing::MetrikiLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;

fn main() {
    env_logger::init();
    LogReporterBuilder::default()
        .registry(global_registry())
        .interval_secs(2)
        .build()
        .unwrap()
        .start();

    let layer = MetrikiLayer::new(global_registry());
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    loop {
        let span = tracing::span!(Level::INFO, "demo");
        let _enter = span.enter();

        tracing::info!("demo_sleep");
        std::thread::sleep(Duration::from_millis(1000));
    }
}
