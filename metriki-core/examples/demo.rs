use std::error::Error;

use metriki_core::MetricsRegistry;

macro_rules! do_times {
    ($times:literal, { $($body:stmt;)* }) => {
        for _ in 0..$times {
            $( $body )*
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // create a metrics registry
    let mr = MetricsRegistry::new();

    // meter demo

    do_times!(9, {
        mr.meter("example.meter").mark();
    });

    println!(
        "count for example.meter: {}",
        mr.meter("example.meter").count()
    );
    println!(
        "m1 rate for example.meter: {}",
        mr.meter("example.meter").m1_rate()
    );

    // counter demo
    do_times!(20, {
        mr.counter("example.counter").inc(1);
    });
    println!(
        "count for example.counter: {}",
        mr.counter("example.counter").value()
    );

    // histogram
    do_times!(1000, {
        mr.histogram("example.histogram")
            .update((rand::random::<f64>() * 100f64) as i64);
    });
    let snapshot = mr.histogram("example.histogram").snapshot();
    println!("p90 for example.histogram: {}", snapshot.quantile(0.9));

    Ok(())
}
