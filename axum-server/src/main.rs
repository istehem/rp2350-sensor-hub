use axum::{
    Json,
    Router,
    //extract::{Path, State},
    extract::State,
    //http::{header, StatusCode},
    http::StatusCode,
    response::{Html, IntoResponse, Response, Result},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::signal::unix::{SignalKind, signal};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static-content");

#[derive(Deserialize)]
struct CreateMeasurement {
    temperature: f64,
    humidity: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
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
            Self::NotFound => {
                let message = "No measurement available yet.";
                warn!("{}", message);
                (StatusCode::NOT_FOUND, message)
            }
            Self::Unreadable => {
                let message = "Couldn't acquire the measurement lock.";
                error!("{}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };
        (
            status,
            axum::Json(serde_json::json!({ "message": message })),
        )
            .into_response()
    }
}

#[derive(Debug)]
enum StaticContentError {
    NotFound,
    InvalidEncoding,
}

impl IntoResponse for StaticContentError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => {
                let message = "File Not Found";
                debug!("{}", message);
                (StatusCode::NOT_FOUND, message)
            }
            Self::InvalidEncoding => {
                let message = "UTF-8 Encoding Error";
                error!("{}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };
        (status, message).into_response()
    }
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        latest_measurement: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/", get(serve_index))
        //.route("/static/*path", get(serve_static))
        .route("/api/measurements/latest", get(latest_measurement))
        .route("/api/measurements", post(create_measurement))
        .with_state(state);

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
    debug!("new measurement: {:?}", measurement);

    Ok((StatusCode::CREATED, Json(measurement)))
}

/*
   async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
   let path = path.trim_start_matches('/');
   let file = STATIC_DIR.get_file(path).ok_or(StatusCode::NOT_FOUND).unwrap();
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    ([(header::CONTENT_TYPE, &mime)], file.contents())
}
*/

async fn serve_index() -> Result<Html<&'static str>, StaticContentError> {
    let file = STATIC_DIR
        .get_file("index.html")
        .ok_or(StaticContentError::NotFound)?;
    Ok(Html(
        file.contents_utf8()
            .ok_or(StaticContentError::InvalidEncoding)?,
    ))
}
