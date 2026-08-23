use crate::{AppState, Measurement, MeasurementError, Params};
use axum::{Json, extract::State, response::Result};
use axum_extra::extract::OptionalQuery;
use ringbuffer::RingBuffer;

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
