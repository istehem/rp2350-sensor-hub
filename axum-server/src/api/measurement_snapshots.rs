use crate::utils::chunk;
use crate::{AppState, Measurement, MeasurementError, Params};
use axum::{Json, extract::State, response::Result};
use axum_extra::extract::OptionalQuery;
use chrono::{DateTime, Utc};
use medians::{Median, Medians};
use ringbuffer::RingBuffer;
use serde::Serialize;
use std::cmp::{Ordering, max};

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Band {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct MedianAndBand {
    pub date: DateTime<Utc>,
    pub median: f64,
    pub band: Band,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct MeasurementSnapshot {
    pub humidity: MedianAndBand,
    pub temperature: MedianAndBand,
}

pub async fn query_measurement_snapshots(
    State(state): State<AppState>,
    OptionalQuery(params): OptionalQuery<Params>,
) -> Result<Json<Vec<MeasurementSnapshot>>, MeasurementError> {
    let measurements_guard = state
        .measurements
        .lock()
        .map_err(|_| MeasurementError::Unreadable)?;

    let mut number_of_chunks = 50;

    if let Some(Params {
        downsample: Some(wanted_count),
    }) = params
    {
        number_of_chunks = wanted_count;
    }

    let measurements: Vec<Measurement> = measurements_guard.iter().copied().collect();
    let snapshots = chunk::split_into_n_chunks(&measurements, max(number_of_chunks, 1))
        .filter_map(calculate_snapshot)
        .collect();

    Ok(Json(snapshots))
}

fn calculate_snapshot(measurements: &[Measurement]) -> Option<MeasurementSnapshot> {
    let temperatures: Vec<f64> = measurements
        .iter()
        .map(|measurement| measurement.temperature)
        .collect();
    let minimun_temperature = temperatures
        .iter()
        .copied()
        .reduce(f64::min)
        .filter(|v| v.is_finite())?;
    let maximum_temperature = temperatures
        .iter()
        .copied()
        .reduce(f64::max)
        .filter(|v| v.is_finite())?;

    let mut compare_temperatures = |a: &Measurement, b: &Measurement| {
        a.temperature
            .partial_cmp(&b.temperature)
            .unwrap_or(Ordering::Greater)
    };
    let median_temperature_result = measurements.median_by(&mut compare_temperatures);
    let median_temperature = match median_temperature_result {
        Ok(Medians::Odd(entry)) => Some((entry.date, entry.temperature)),
        Ok(Medians::Even((lower, upper))) => {
            Some((lower.date, (lower.temperature + upper.temperature) / 2.0))
        }

        _ => None,
    }?;

    let humidities: Vec<f64> = measurements
        .iter()
        .map(|measurement| measurement.humidity)
        .collect();
    let mininum_humidity = humidities
        .iter()
        .copied()
        .reduce(f64::min)
        .filter(|v| v.is_finite())?;
    let maximum_humidity = humidities
        .iter()
        .copied()
        .reduce(f64::max)
        .filter(|v| v.is_finite())?;

    let mut compare_humidities = |a: &Measurement, b: &Measurement| {
        a.humidity
            .partial_cmp(&b.humidity)
            .unwrap_or(Ordering::Greater)
    };
    let median_humidity_result = measurements.median_by(&mut compare_humidities);
    let median_humidity = match median_humidity_result {
        Ok(Medians::Odd(entry)) => Some((entry.date, entry.humidity)),
        Ok(Medians::Even((lower, upper))) => {
            Some((lower.date, (lower.humidity + upper.humidity) / 2.0))
        }
        _ => None,
    }?;

    let median_and_band_temperature = MedianAndBand {
        date: median_temperature.0,
        median: median_temperature.1,
        band: Band {
            minimum: minimun_temperature,
            maximum: maximum_temperature,
        },
    };

    let median_and_band_humidity = MedianAndBand {
        date: median_humidity.0,
        median: median_humidity.1,
        band: Band {
            minimum: mininum_humidity,
            maximum: maximum_humidity,
        },
    };

    Some(MeasurementSnapshot {
        humidity: median_and_band_humidity,
        temperature: median_and_band_temperature,
    })
}
