use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Method, StatusCode, Uri, header},
    response::{Html, IntoResponse, Response, Result},
    routing::{get, post},
};
use axum_extra::{
    TypedHeader,
    extract::OptionalQuery,
    headers::{Authorization, authorization::Basic},
};
use chrono::{DateTime, Utc};
use include_dir::{Dir, include_dir};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::signal::unix::{SignalKind, signal};
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

static STATIC_CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static-content");

const USER: &str = env!("REST_USER");
const PASSWORD: &str = env!("REST_USER_PASSWORD");

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
    measurements: Arc<Mutex<AllocRingBuffer<Measurement>>>,
}

#[derive(Debug)]
enum MeasurementError {
    NotFound,
    Unreadable,
    Unauthorized,
}

#[derive(Deserialize)]
struct Params {
    downsample: Option<usize>,
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
            Self::Unauthorized => {
                let message = "Request was unauthorized.";
                warn!("{}", message);
                (StatusCode::UNAUTHORIZED, message)
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
        measurements: Arc::new(Mutex::new(AllocRingBuffer::new(5000))),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(index))
        .route("/static-content/{*param}", get(static_content))
        .route("/api/measurements/latest", get(latest_measurement))
        .route("/api/measurements", get(query_measurements))
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

async fn latest_measurement(
    State(state): State<AppState>,
) -> Result<Json<Measurement>, MeasurementError> {
    let measurements = state
        .measurements
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;

    match measurements.back() {
        Some(measurements) => Ok(Json(*measurements)),
        None => Err(MeasurementError::NotFound),
    }
}

// decimation with interval offset
fn downsample_measurements(
    measurements: Vec<Measurement>,
    wanted_count: usize,
) -> Vec<Measurement> {
    if measurements.is_empty() || wanted_count == 0 || wanted_count >= measurements.len() {
        return measurements;
    }
    let mut picked = Vec::with_capacity(wanted_count);
    let interval = measurements.len() as f64 / wanted_count as f64;

    for i in 0..wanted_count {
        let wanted_index = ((i as f64 * interval + interval / 2.0).floor()) as usize;
        let index = wanted_index.min(measurements.len() - 1);
        picked.push(measurements[index]);
    }

    picked
}

async fn query_measurements(
    State(state): State<AppState>,
    OptionalQuery(params): OptionalQuery<Params>,
) -> Result<Json<Vec<Measurement>>, MeasurementError> {
    let measurements_guard = state
        .measurements
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;
    let mut measurements = measurements_guard.iter().copied().collect();
    if let Some(Params {
        downsample: Some(wanted_count),
    }) = params
    {
        measurements = downsample_measurements(measurements, wanted_count);
    }

    Ok(Json(measurements))
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
