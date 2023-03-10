use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use metriki_core::MetricsRegistry;
use metriki_log_reporter::LogReporterBuilder;
#[cfg(feature = "rt")]
use metriki_tokio::TokioRuntimeMetricsSetBuilder;
use metriki_tokio::TokioTaskMetricsSetBuilder;
#[cfg(feature = "rt")]
use tokio_metrics::RuntimeMonitor;
use tokio_metrics::TaskMonitor;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let registry = MetricsRegistry::arc();
    LogReporterBuilder::default()
        .registry(registry.clone())
        .interval_secs(10)
        .build()
        .unwrap()
        .start();

    let task_monitor = TaskMonitor::new();
    let task_metrics_set = TokioTaskMetricsSetBuilder::default()
        .name("service") // TODO: fixme
        .monitor(&task_monitor)
        .build()
        .unwrap();
    registry.register_metrics_set(&task_metrics_set.name().clone(), Arc::new(task_metrics_set));
    #[cfg(feature = "rt")]
    {
        let handle = tokio::runtime::Handle::current();
        let runtime_monitor = RuntimeMonitor::new(&handle);
        let runtime_metrics_set = TokioRuntimeMetricsSetBuilder::default()
            .name("current_runtime")
            .monitor(&runtime_monitor)
            .build()
            .unwrap();
        registry.register_metrics_set(
            &runtime_metrics_set.name().clone(),
            Arc::new(runtime_metrics_set),
        );
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // TaskMonitor to instrument this
    let make_svc = make_service_fn(|_conn| {
        task_monitor.instrument(async { Ok::<_, Infallible>(service_fn(hello_world)) })
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening 3001");

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
