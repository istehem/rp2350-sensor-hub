use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Method, StatusCode, Uri, header},
    response::{Html, IntoResponse, Response, Result},
    routing::{get, post},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use axum_server::api::{measurement_snapshots, measurements};
use axum_server::{AppState, Measurement, MeasurementError};
use chrono::Utc;
use include_dir::{Dir, include_dir};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::signal::unix::{SignalKind, signal};
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

static STATIC_CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static-content");

const USER: &str = env!("REST_USER");
const PASSWORD: &str = env!("REST_USER_PASSWORD");

#[derive(Deserialize)]
struct CreateMeasurement {
    temperature: f64,
    humidity: f64,
}

#[derive(Serialize)]
struct Version {
    version: String,
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
        measurements: Arc::new(Mutex::new(AllocRingBuffer::new(5000))),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(index))
        .route("/static-content/{*param}", get(static_content))
        .route("/api/version", get(version))
        .route(
            "/api/measurements/latest",
            get(measurements::latest_measurement),
        )
        .route("/api/measurements", get(measurements::query_measurements))
        .route(
            "/api/measurement-snapshots",
            get(measurement_snapshots::query_measurement_snapshots),
        )
        .route("/api/measurements", post(create_measurement))
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

async fn version() -> Json<Version> {
    Json(Version {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

fn validate_authorization(
    auth: Option<TypedHeader<Authorization<Basic>>>,
) -> Result<(), MeasurementError> {
    let credentials = match auth {
        Some(TypedHeader(Authorization(basic))) => basic,
        None => {
            return Err(MeasurementError::Unauthorized);
        }
    };

    let username = credentials.username();
    let password = credentials.password();
    if username != USER || password != PASSWORD {
        return Err(MeasurementError::Unauthorized);
    }
    Ok(())
}

async fn create_measurement(
    auth: Option<TypedHeader<Authorization<Basic>>>,
    State(state): State<AppState>,
    Json(payload): Json<CreateMeasurement>,
) -> Result<(StatusCode, Json<Measurement>), MeasurementError> {
    validate_authorization(auth)?;

    let measurement = Measurement {
        date: Utc::now(),
        temperature: payload.temperature,
        humidity: payload.humidity,
    };
    let mut measurements = state
        .measurements
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;
    measurements.enqueue(measurement);
    debug!("new measurement: {:?}", measurement);

    Ok((StatusCode::CREATED, Json(measurement)))
}

async fn static_content(Path(path): Path<String>) -> Result<impl IntoResponse, StaticContentError> {
    let path = path.trim_start_matches('/');
    let file = STATIC_CONTENT_DIR
        .get_file(path)
        .ok_or(StaticContentError::NotFound)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    Ok((
        [(header::CONTENT_TYPE, mime.as_ref().to_string())],
        file.contents(),
    ))
}

async fn index() -> Result<Html<&'static str>, StaticContentError> {
    let file = STATIC_CONTENT_DIR
        .get_file("index.html")
        .ok_or(StaticContentError::NotFound)?;
    Ok(Html(
        file.contents_utf8()
            .ok_or(StaticContentError::InvalidEncoding)?,
    ))
}
