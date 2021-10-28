fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &["opentelemetry-proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto"],
            &["opentelemetry-proto/"],
        )
        .unwrap();
}
