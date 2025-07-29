use axum::http::StatusCode;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

pub fn cache_image_filename(filename: &String) -> String {
    let encoded = URL_SAFE.encode(filename.as_bytes());

    format!("{encoded}.webp")
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("Unhandled internal error: {:#?}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong".to_string(),
    )
}
