use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response, Result},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
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
    date: DateTime<Utc>,
    temperature: f64,
    humidity: f64,
}
#[derive(Clone)]
struct AppState {
    latest_measurement: Arc<Mutex<Option<Measurement>>>,
}

#[derive(Debug)]
enum MeasurementError {
    NotFound,
    Unreadable,
}

impl IntoResponse for MeasurementError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "No measurement available yet."),
            Self::Unreadable => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't acquire the measurement lock.",
            ),
        };
        (status, axum::Json(serde_json::json!({ "msg": message }))).into_response()
    }
}

#[tokio::main]
async fn main() {
    let state = AppState {
        latest_measurement: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/api/measurements/latest", get(latest_measurement))
        .route("/api/measurements", post(create_measurement))
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
) -> Result<Json<Measurement>, MeasurementError> {
    let latest_measurement = state
        .latest_measurement
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;

    latest_measurement
        .map(Json)
        .ok_or(MeasurementError::NotFound)
}

async fn create_measurement(
    State(state): State<AppState>,
    Json(payload): Json<CreateMeasurement>,
) -> Result<(StatusCode, Json<Measurement>), MeasurementError> {
    let measurement = Measurement {
        date: Utc::now(),
        temperature: payload.temperature,
        humidity: payload.humidity,
    };
    let mut latest_measurement = state
        .latest_measurement
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;
    *latest_measurement = Some(measurement);

    Ok((StatusCode::CREATED, Json(measurement)))
}
