use metriki_core::MetricsRegistry;
use metriki_tracing::MetrikiSubscriber;
use tracing;

fn main() -> Result<(), Box<Error>> {
    MetrikiSubscriber::new().init();
}
