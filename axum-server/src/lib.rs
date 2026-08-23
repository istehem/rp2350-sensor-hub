use chrono::{DateTime, Utc};
use ringbuffer::AllocRingBuffer;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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

#[derive(Deserialize)]
pub struct Params {
    pub downsample: Option<usize>,
}

pub mod utils {
    pub mod chunk;
}

pub mod api {
    pub mod error;
    pub mod measurement_snapshots;
    pub mod measurements;
    pub mod version;
}

pub mod static_content;
