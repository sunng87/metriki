use std::error::Error;

use metriki_core::global::global_registry;
use metriki_tracing::MetrikiLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;

fn main() -> Result<(), Box<dyn Error>> {
    let layer = MetrikiLayer::new(global_registry());
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let span = tracing::span!(Level::INFO, "demo");

    drop(span);

    Ok(())
}
