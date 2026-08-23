use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response, Result},
};
use include_dir::{Dir, include_dir};
use tracing::{debug, error};

static STATIC_CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static-content");

#[derive(Debug)]
pub enum StaticContentError {
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

pub async fn static_content(
    Path(path): Path<String>,
) -> Result<impl IntoResponse, StaticContentError> {
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

pub async fn index() -> Result<Html<&'static str>, StaticContentError> {
    let file = STATIC_CONTENT_DIR
        .get_file("index.html")
        .ok_or(StaticContentError::NotFound)?;
    Ok(Html(
        file.contents_utf8()
            .ok_or(StaticContentError::InvalidEncoding)?,
    ))
}
