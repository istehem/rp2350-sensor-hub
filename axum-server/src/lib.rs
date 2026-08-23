use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use ringbuffer::AllocRingBuffer;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tracing::{error, warn};

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Measurement {
    pub date: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub measurements: Arc<Mutex<AllocRingBuffer<Measurement>>>,
}

#[derive(Debug)]
pub enum MeasurementError {
    NotFound,
    Unreadable,
    Unauthorized,
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

pub mod utils {
    pub mod chunk;
}

pub mod api {
    pub mod measurements;
}
