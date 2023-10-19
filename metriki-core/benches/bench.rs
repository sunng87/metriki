#![feature(test)]
extern crate test;


use test::Bencher;
use threadpool::ThreadPool;

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

#[bench]
fn bench_multithread(b: &mut Bencher) {
    let pool = ThreadPool::new(16);

    b.iter(|| {
        let rg = MetricsRegistry::arc();
        for _ in 0..1000 {
            let rg2 = rg.clone();
            pool.execute(move || {
                let name = format!("test.meter.{}", (rand::random::<f64>() * 100.0) as u64);
                rg2.meter(&name).mark();
            });
        }
        pool.join();
    });
}
