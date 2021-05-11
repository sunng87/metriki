use std::error::Error;

use metriki_core::global::global_registry;

macro_rules! do_times {
    ($times:literal, { $($body:stmt;)* }) => {
        for _ in 0..$times {
            $( $body )*
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // meter demo

    do_times!(9, {
        global_registry().meter("example.meter").mark();
    });

    println!(
        "count for example.meter: {}",
        global_registry().meter("example.meter").count()
    );
    println!(
        "m1 rate for example.meter: {}",
        global_registry().meter("example.meter").m1_rate()
    );

    // counter demo
    do_times!(20, {
        global_registry().counter("example.counter").inc(1);
    });
    println!(
        "count for example.counter: {}",
        global_registry().counter("example.counter").value()
    );

    // histogram
    do_times!(1000, {
        global_registry()
            .histogram("example.histogram")
            .update((rand::random::<f64>() * 100f64) as u64);
    });
    let snapshot = global_registry().histogram("example.histogram").snapshot();
    println!("p90 for example.histogram: {}", snapshot.quantile(0.9));

    Ok(())
}
