use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub version: String,
}

pub async fn version() -> Json<Version> {
    Json(Version {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
