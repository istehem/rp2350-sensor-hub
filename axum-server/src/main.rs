use axum::{
    Router,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
};
use axum_server::AppState;
use axum_server::api::{measurement_snapshots, measurements, version};
use axum_server::static_content;
use ringbuffer::AllocRingBuffer;
use std::sync::{Arc, Mutex};
use tokio::signal::unix::{SignalKind, signal};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        measurements: Arc::new(Mutex::new(AllocRingBuffer::new(5000))),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(static_content::index))
        .route(
            "/static-content/{*param}",
            get(static_content::static_content),
        )
        .route("/api/version", get(version::version))
        .route(
            "/api/measurements/latest",
            get(measurements::latest_measurement),
        )
        .route("/api/measurements", get(measurements::query_measurements))
        .route(
            "/api/measurement-snapshots",
            get(measurement_snapshots::query_measurement_snapshots),
        )
        .route("/api/measurements", post(measurements::create_measurement))
        .with_state(state)
        .fallback(fallback)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    info!("⚡️Server will listen to port: 5000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).expect("failed to install signal handler");
    sigterm.recv().await;
    info!("SIGTERM received, shutting down...");
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    let message = format!("No such route: {}", uri.path());
    (
        StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({ "message": message })),
    )
}
