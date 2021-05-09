#![feature(test)]
extern crate test;

use rand;
use test::Bencher;

use metriki_core::MetricsRegistry;

#[bench]
fn bench_meter_mark(b: &mut Bencher) {
    let rg = MetricsRegistry::arc();

    b.iter(|| rg.meter("test.meter").mark());
}

#[bench]
fn bench_histogram_update(b: &mut Bencher) {
    let rg = MetricsRegistry::arc();

    b.iter(|| {
        rg.histogram("test.histogram")
            .update((rand::random::<f64>() * 1000.0) as u64)
    });
}

#[bench]
fn bench_timer(b: &mut Bencher) {
    let rg = MetricsRegistry::arc();

    b.iter(|| {
        let timer = rg.timer("test.timer");
        let ctx = timer.start();
        ctx.stop();
    });
}
