use crate::{AppState, Measurement, MeasurementError, Params};
use axum::{Json, extract::State, http::StatusCode, response::Result};
use axum_extra::{
    TypedHeader,
    extract::OptionalQuery,
    headers::{Authorization, authorization::Basic},
};
use chrono::Utc;
use ringbuffer::RingBuffer;
use serde::Deserialize;
use tracing::debug;

const USER: &str = env!("REST_USER");
const PASSWORD: &str = env!("REST_USER_PASSWORD");

#[derive(Deserialize)]
pub struct CreateMeasurement {
    pub temperature: f64,
    pub humidity: f64,
}

pub async fn create_measurement(
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

pub async fn latest_measurement(
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

pub async fn query_measurements(
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
