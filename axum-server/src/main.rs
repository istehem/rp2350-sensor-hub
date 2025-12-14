use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::signal::unix::{SignalKind, signal};

#[derive(Deserialize)]
struct CreateMeasurement {
    temperature: f64,
    humidity: f64,
}

#[derive(Clone, Copy, Serialize)]
struct Measurement {
    id: u64,
    temperature: f64,
    humidity: f64,
}
#[derive(Clone)]
struct AppState {
    latest_measurement: Arc<Mutex<Option<Measurement>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        latest_measurement: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/api/measurements/latest", get(latest_measurement))
        .route("/api/measurements/", post(create_measurement))
        .with_state(state);

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

async fn latest_measurement(
    State(state): State<AppState>,
) -> Result<Json<Measurement>, StatusCode> {
    let latest_measurement = state
        .latest_measurement
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    latest_measurement.map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn create_measurement(
    State(state): State<AppState>,
    Json(payload): Json<CreateMeasurement>,
) -> Result<(StatusCode, Json<Measurement>), StatusCode> {
    let measurement = Measurement {
        id: 1337,
        temperature: payload.temperature,
        humidity: payload.humidity,
    };
    let mut latest_measurement = state
        .latest_measurement
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    *latest_measurement = Some(measurement);

    Ok((StatusCode::CREATED, Json(measurement)))
}
