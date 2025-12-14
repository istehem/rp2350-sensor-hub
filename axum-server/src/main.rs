use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::signal::unix::{SignalKind, signal};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(create_measurement));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).expect("failed to install signal handler");
    sigterm.recv().await;
    println!("SIGTERM received, shutting down...");
}

async fn create_measurement(
    Json(payload): Json<CreateMeasurement>,
) -> (StatusCode, Json<Measurement>) {
    let measurement = Measurement {
        id: 1337,
        temperature: payload.temperature,
        humidity: payload.humidity,
    };
    (StatusCode::CREATED, Json(measurement))
}

#[derive(Deserialize)]
struct CreateMeasurement {
    temperature: f64,
    humidity: f64,
}

#[derive(Serialize)]
struct Measurement {
    id: u64,
    temperature: f64,
    humidity: f64,
}
