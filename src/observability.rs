use axum::Router;
use tracing::Level;

use tower_http::request_id::{MakeRequestId, RequestId, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use uuid::Uuid;

#[derive(Default, Clone)]
struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(
        &mut self,
        _: &axum::http::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        Uuid::new_v4().to_string().parse().ok().map(RequestId::new)
    }
}

pub(crate) fn init_tracing() {
    use opentelemetry_otlp::WithExportConfig;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::Registry;

    // Create a new OpenTelemetry pipeline
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://otlp-opentelemetry-collector:4317");
    // Then pass it into pipeline builder
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default()
        .with(telemetry)
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub(crate) fn make_observable(app: Router<axum::body::Body>) -> Router<axum::body::Body> {
    app.layer(
        TraceLayer::new_for_http().make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(true),
        ),
    )
    .layer(SetRequestIdLayer::x_request_id(UuidRequestId::default()))
}
